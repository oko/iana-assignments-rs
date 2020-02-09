#!/opt/chef/embedded/bin/ruby
require 'optparse'
require 'csv'
require 'net/http'
require 'erubis'
require 'ostruct'

functions = {}

class String
  def normalize
    self.gsub(/[-\s]+/, "_").gsub(/[^_a-zA-Z0-9]+/, "").strip.upcase
  end
  
  def arch_type_to_i
    self.strip.gsub('0x', '').gsub(' ', '').to_i(16)
  end

  def underscore
    self.gsub(/::/, '/').
    gsub(/([A-Z]+)([A-Z][a-z])/,'\1_\2').
    gsub(/([a-z\d])([A-Z])/,'\1_\2').
    tr("-", "_").
    downcase
  end
end

dhcpv6messagetypes = Proc.new {
  rows = CSV.new(
    Net::HTTP.get(
      URI('https://www.iana.org/assignments/dhcpv6-parameters/dhcpv6-parameters-1.csv')
    )
  ).read.map do |row|
    OpenStruct.new(value: row[0].to_i, desc: row[1].normalize, ref: row[2])
  end.select do |row|
    !['RESERVED', 'UNASSIGNED'].include? row.desc
  end
  {:rows => rows[1..]}
}
functions['dhcpv6.messagetypes'] = dhcpv6messagetypes

dhcpv6optioncodes = Proc.new {
  rows = CSV.new(
    Net::HTTP.get(
      URI('https://www.iana.org/assignments/dhcpv6-parameters/dhcpv6-parameters-2.csv')
    )
  ).read.map do |row|
    OpenStruct.new(value: row[0].to_i, desc: row[1].normalize, oro: row[2], single: row[3], ref: row[4])
  end.select do |row|
    !['RESERVED', 'UNASSIGNED'].include? row.desc
  end
  {:rows => rows[1..]}
}
functions['dhcpv6.optioncodes'] = dhcpv6optioncodes

dhcpv6statuscodes = Proc.new {
  rows = CSV.new(
    Net::HTTP.get(
      URI('https://www.iana.org/assignments/dhcpv6-parameters/dhcpv6-parameters-5.csv')
    )
  ).read.map do |row|
    OpenStruct.new(value: row[0].to_i, desc: row[1].strip.underscore.normalize, ref: row[2])
  end.select do |row|
    !['RESERVED', 'UNASSIGNED'].include? row.desc
  end
  {:rows => rows[1..]}
}
functions['dhcpv6.statuscodes'] = dhcpv6statuscodes

dhcpv6duidtypes = Proc.new {
  rows = CSV.new(
    Net::HTTP.get(
      URI('https://www.iana.org/assignments/dhcpv6-parameters/dhcpv6-parameters-6.csv')
    )
  ).read.map do |row|
    OpenStruct.new(value: row[0].to_i, desc: row[1].normalize, ref: row[2])
  end.select do |row|
    !['RESERVED', 'UNASSIGNED'].include? row.desc
  end
  {:rows => rows[1..]}
}
functions['dhcpv6.duidtypes'] = dhcpv6duidtypes

dhcpv6opt_lq_query_types = Proc.new {
  rows = CSV.new(
    Net::HTTP.get(
      URI('https://www.iana.org/assignments/dhcpv6-parameters/dhcpv6-parameters-7.csv')
    )
  ).read.map do |row|
    OpenStruct.new(value: row[0].to_i, desc: row[1].normalize, ref: row[2])
  end.select do |row|
    !['RESERVED', 'UNASSIGNED'].include? row.desc
  end
  {:rows => rows[1..]}
}
functions['dhcpv6.opt_lq_query_types'] = dhcpv6opt_lq_query_types

dhcpv6ieee80221servicetypes = Proc.new {
  rows = CSV.new(
    Net::HTTP.get(
      URI('https://www.iana.org/assignments/dhcpv6-parameters/ieee-80221-service-type.csv')
    )
  ).read.map do |row|
    OpenStruct.new(value: row[0].to_i, desc: row[1].normalize, ref: row[2])
  end.select do |row|
    !['RESERVED', 'UNASSIGNED'].include? row.desc
  end
  {:rows => rows[1..]}
}
functions['dhcpv6.ieee80221servicetypes'] = dhcpv6ieee80221servicetypes

dhcpv6archtypes = Proc.new {
  rows = CSV.new(
    Net::HTTP.get(
      URI('https://www.iana.org/assignments/dhcpv6-parameters/processor-architecture.csv')
    )
  ).read.map do |row|
    OpenStruct.new(value: row[0].arch_type_to_i, desc: row[1].sub('(DEPRECATED)', '').strip.normalize, ref: row[2])
  end.select do |row|
    !['RESERVED', 'UNASSIGNED'].include? row.desc
  end
  {:rows => rows[1..]}
}
functions['dhcpv6.archtypes'] = dhcpv6archtypes

dhcpv6ntp_tsrc_subopt_types = Proc.new {
  rows = CSV.new(
    Net::HTTP.get(
      URI('https://www.iana.org/assignments/dhcpv6-parameters/ntp-time-source.csv')
    )
  ).read.map do |row|
    OpenStruct.new(value: row[0].to_i, desc: row[1].strip, ref: row[2])
  end.select do |row|
    !['RESERVED', 'UNASSIGNED'].include? row.desc.normalize
  end
  {:rows => rows[1..]}
}
functions['dhcpv6.ntp_tsrc_subopt_types'] = dhcpv6ntp_tsrc_subopt_types

arphwtypes = Proc.new {
  rows = CSV.new(
    Net::HTTP.get(
      URI('https://www.iana.org/assignments/arp-parameters/arp-parameters-2.csv')
    ).sub("\nTR", " ")
  ).read.map do |row|
    OpenStruct.new(value: row[0].to_i, desc: row[1].strip.underscore.normalize, ref: row[2])
  end.select do |row|
    !['RESERVED', 'UNASSIGNED'].include? row.desc
  end
  rct = {}
  rows.each do |row|
    if rct.key? row.desc
      rct[row.desc] += 1
      row.desc = "#{row.desc}_#{rct[row.desc]}"
    else
      rct[row.desc] = 0
    end
  end
  {:rows => rows[1..]}
}
functions['arp.hwtypes'] = arphwtypes

if $PROGRAM_NAME == __FILE__
  options = {
    :type => 'rs'
  }
  OptionParser.new do |opts|
    opts.banner = "usage: ianagen.rb <registry>"
    opts.on('-t', '--type', 'file type') do |v|
      options[:type] = v
    end
  end.parse!

  regs = {}

  argv = ARGV
  puts argv
  if argv.length.zero?
    argv = functions.keys
  end

  argv.each do |arg|
    if arg.include? ':'
      registry, template = arg.split(arg, ':', 2)
    else
      registry = arg
      template = File.join('tools/templates', "#{arg}.#{options[:type]}.erb")
    end
    regs[registry] = template
  end

  regs.each do |reg, tpl|
    fn = functions[reg]
    data = Erubis::Eruby.new(File.read(tpl)).result(fn.call)
    mod, file = reg.split('.')
    fj = File.join('src', mod, "#{file}.rs")
    File.write(fj, data)
    puts fj
  end
end
