module Jobs
  Install = 'install'
  Skipped = 'skipped'
  Building = 'building'
  Clearing = 'clearing'
  Checks = 'checks'
  Test = 'Test'
  Clippy = 'clippy'
end

module Owner
  Extractor = 'Extractor'
  Lib = 'Lib'
end

class Reporter
  @jobs = []
  def self.add(type, owner, description, icon)
    @jobs.push({
                 'type' => type,
                 'owner' => owner,
                 'description' => description,
                 'icon' => icon
               })
    # unless Shell.is_verbose_hidden
    #   puts "#{icon_type(type)}\t[#{align(type,
    #                                      10)}]\t[#{align(owner, 10)}]: #{description}"
    # end
  end

  def self.print
    @jobs.each do |job|
      puts "#{icon_type(job['type'])}\t[#{align(job['type'],
                                                10)}]\t[#{align(job['owner'], 10)}]: #{job['description']}"
    end
  end
end

def align(content, len)
  spaces = len - content.length
  spaces = 0 if spaces < 0
  "#{content}#{' ' * spaces}"
end

def icon_type(type)
  case type
  when Jobs::Install
    '*'
  when Jobs::Skipped
    '*'
  when Jobs::Building
    '*'
  when Jobs::Clearing
    '*'
  when Jobs::Checks
    '*'
  when Jobs::Clippy
    '*'
  when Jobs::Test
    '*'
  when Jobs::Other
    '*'
  else
    '...'
  end
end
