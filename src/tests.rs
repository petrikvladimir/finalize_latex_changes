use super::*;

#[test]
fn test_defaults() {
    let f = Filter { ..Default::default() };
    assert_eq!(f.counter_added, 0);
    assert_eq!(f.counter_replaced, 0);
    assert_eq!(f.counter_deleted, 0);
}

#[test]
fn test_added() {
    let mut f = Filter { ..Default::default() };
    assert_eq!(f.process("\\added{added_text}"), "added_text");
    assert_eq!(f.process("asdf\\added{added_text}adsf"), "asdfadded_textadsf");
    assert_eq!(f.process("asdf \\added{added_text}adsf"), "asdf added_textadsf");
    assert_eq!(f.process("asdf \\add ed{added_text}adsf"), "asdf \\add ed{added_text}adsf");
    assert_eq!(f.process("asdf \\added{added{}_text\\italic{adfs}}adsf"), "asdf added{}_text\\italic{adfs}adsf");
    assert_eq!(f.process("\\added{added_text}\\added{added_text}"), "added_textadded_text");
    assert_eq!(f.counter_added, 6);
}

#[test]
fn test_deleted() {
    let mut f = Filter { ..Default::default() };
    assert_eq!(f.process("\\deleted{wrong text}"), "");
    assert_eq!(f.process("\\deleted{wrong text}goal"), "goal");
    assert_eq!(f.process("goal\\deleted{wrong text}"), "goal");
    assert_eq!(f.process("go\\deleted{wrong text}al"), "goal");
    assert_eq!(f.process("\\deleted{wrong \\textit{another of \\textbf{another}} text}goal"), "goal");
    assert_eq!(f.counter_deleted, 5);
}

#[test]
fn test_replaced() {
    let mut f = Filter { ..Default::default() };
    assert_eq!(f.process("\\replaced{good}{wrong}"), "good");
    assert_eq!(f.process("\\replaced{goo}{wrong}d"), "good");
    assert_eq!(f.process("g\\replaced{ood}{wrong}"), "good");
    assert_eq!(f.process("\\replaced{good}{wr\\textit{asdf}ong}"), "good");
    assert_eq!(f.process("\\replaced{g\\textit{asd}ood}{wr\\textit{asdf}ong}"), "g\\textit{asd}ood");
    assert_eq!(f.counter_replaced, 5);
}

#[test]
fn test_process_file_same_file() {
    let mut f = Filter::new();
    let out = f.process_file(Path::new("asdf"), Path::new("asdf"));
    assert!(out.is_err());
}

#[test]
fn test_count_open_brackets() {
    let mut f = Filter::new();
    assert_eq!(f.count_open_brackets('a'), 0);
    assert_eq!(f.count_open_brackets('{'), 1);
    assert_eq!(f.count_open_brackets('{'), 2);
    assert_eq!(f.count_open_brackets('}'), 1);
    assert_eq!(f.count_open_brackets('{'), 2);
    assert_eq!(f.count_open_brackets('{'), 3);
    assert_eq!(f.count_open_brackets('}'), 2);
    assert_eq!(f.count_open_brackets('}'), 1);
    assert_eq!(f.count_open_brackets('}'), 0);
}

#[test]
fn test_create_steps_for_command() {
    let mut f = Filter::new();
    assert_eq!(f.create_steps_for_command("\\adaded"), false);
    assert_eq!(f.counter_added, 0);
    assert_eq!(f.create_steps_for_command("\\added"), true);
    assert_eq!(f.counter_added, 1);
    assert_eq!(f.create_steps_for_command("\\deleted"), true);
    assert_eq!(f.create_steps_for_command("\\deleted"), true);
    assert_eq!(f.counter_deleted, 2);
    assert_eq!(f.create_steps_for_command("\\replaced"), true);
    assert_eq!(f.counter_replaced, 1);
}

#[test]
fn test_multi_line() {
    let mut f = Filter::new();
    assert_eq!(f.process("\\added{added_text"), "added_text");
    assert_eq!(f.process("  asdf"), "  asdf");
    assert_eq!(f.process("}"), "");
    assert_eq!(f.process("\\replaced[id = vp] {"), "");
    assert_eq!(f.process("  new "), "  new ");
    assert_eq!(f.process("  text %"), "  text %");
    assert_eq!(f.process("}{old text}"), "");
}
