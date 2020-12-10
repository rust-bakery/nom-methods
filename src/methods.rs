/// Makes a method from a parser combination
///
/// The must be set up because the compiler needs
/// the information
///
/// ```ignore
/// method!(my_function<Parser<'a> >( &[u8] ) -> &[u8], tag!("abcd"));
/// // first type parameter is `self`'s type, second is input, third is output
/// method!(my_function<Parser<'a>, &[u8], &[u8]>,     tag!("abcd"));
/// //prefix them with 'pub' to make the methods public
/// method!(pub my_function<Parser<'a>,&[u8], &[u8]>, tag!("abcd"));
/// ```
#[macro_export(local_inner_macros)]
macro_rules! method (
  // Non-public immutable self
  ($name:ident( $i:ty ) -> $o:ty, &$self_:ident, $submac:ident!( $($args:tt)* )) => (
      #[allow(unused_variables)]
      fn $name( &$self_, i: $i ) -> $crate::lib::IResult<$i,$o,$crate::lib::Error<$i>> {
        let result = $submac!(i, $($args)*);
        result
      }
  );
  ($name:ident<$i:ty,$o:ty,$e:ty>, &$self_:ident, $submac:ident!( $($args:tt)* )) => (
    #[allow(unused_variables)]
    fn $name( &$self_, i: $i ) -> $crate::lib::IResult<$i, $o, $e> {
      let result = $submac!(i, $($args)*);
      result
    }
  );
  ($name:ident<$i:ty,$o:ty>, &$self_:ident, $submac:ident!( $($args:tt)* )) => (
    #[allow(unused_variables)]
    fn $name( &$self_, i: $i ) -> $crate::lib::IResult<$i,$o,$crate::lib::Error<$i>>  {
      let result = $submac!(i, $($args)*);
      result
    }
  );
  ($name:ident<$o:ty>, &$self_:ident, $submac:ident!( $($args:tt)* )) => (
      #[allow(unused_variables)]
      fn $name( &$self_, i: &[u8] ) -> $crate::lib::IResult<&[u8], $o, $crate::lib::Error<&[u8]>> {
        let result = $submac!(i, $($args)*);
        result
      }
  );
  ($name:ident, &$self_:ident, $submac:ident!( $($args:tt)* )) => (
      #[allow(unused_variables)]
      fn $name( &$self_, i: &[u8] ) -> $crate::lib::IResult<&[u8], &[u8], $crate::lib::Error<&[u8]>> {
        let result = $submac!(i, $($args)*);
        result
      }
  );
  // Public immutable self
  (pub $name:ident( $i:ty ) -> $o:ty, &$self_:ident, $submac:ident!( $($args:tt)* )) => (
      #[allow(unused_variables)]
      pub fn $name( &$self_, i: $i ) -> $crate::lib::IResult<$i,$o,$crate::lib::Error<$i>> {
        let result = $submac!(i, $($args)*);
        result
      }
  );
  (pub $name:ident<$i:ty,$o:ty,$e:ty>, &$self_:ident, $submac:ident!( $($args:tt)* )) => (
      #[allow(unused_variables)]
      pub fn $name( &$self_, i: $i ) -> $crate::lib::IResult<$i, $o, $e> {
        let result = $submac!(i, $($args)*);
        result
      }
  );
  (pub $name:ident<$i:ty,$o:ty>, &$self_:ident, $submac:ident!( $($args:tt)* )) => (
    #[allow(unused_variables)]
    pub fn $name( &$self_, i: $i ) -> $crate::lib::IResult<$i,$o,$crate::lib::Error<$i>> {
      let result = $submac!(i, $($args)*);
      result
    }
  );
  (pub $name:ident<$o:ty>, &$self_:ident, $submac:ident!( $($args:tt)* )) => (
    #[allow(unused_variables)]
    pub fn $name( &$self_, i: &[u8] ) -> $crate::lib::IResult<&[u8], $o, $crate::lib::Error<&[u8]>> {
      let result = $submac!(i, $($args)*);
      result
    }
  );
  (pub $name:ident, &$self_:ident, $submac:ident!( $($args:tt)* )) => (
    #[allow(unused_variables)]
    pub fn $name( &$self_, i: &[u8] ) -> $crate::lib::IResult<&[u8], &[u8], $crate::lib::Error<&[u8]>> {
      let result = $submac!(i, $($args)*);
      result
    }
  );
  // Non-public mutable self
  ($name:ident( $i:ty ) -> $o:ty, &mut $self_:ident, $submac:ident!( $($args:tt)* )) => (
      #[allow(unused_variables)]
      fn $name( &mut $self_, i: $i ) -> $crate::lib::IResult<$i,$o,$crate::lib::Error<$i>> {
        let result = $submac!(i, $($args)*);
        result
      }
  );
  ($name:ident<$i:ty,$o:ty,$e:ty>, &mut $self_:ident, $submac:ident!( $($args:tt)* )) => (
      #[allow(unused_variables)]
      fn $name( &mut $self_, i: $i ) -> $crate::lib::IResult<$i, $o, $e> {
        let result = $submac!(i, $($args)*);
        result
      }
  );
  ($name:ident<$i:ty,$o:ty>, &mut $self_:ident, $submac:ident!( $($args:tt)* )) => (
    #[allow(unused_variables)]
    fn $name( &mut $self_, i: $i ) -> $crate::lib::IResult<$i,$o,$crate::lib::Error<$i>> {
      let result = $submac!(i, $($args)*);
      result
    }
  );
  ($name:ident<$o:ty>, &mut $self_:ident, $submac:ident!( $($args:tt)* )) => (
      #[allow(unused_variables)]
      fn $name( &mut $self_, i: &[u8] ) -> $crate::lib::IResult<&[u8], $o, $crate::lib::Error<&[u8]>> {
        let result = $submac!(i, $($args)*);
        result
      }
  );
  ($name:ident, &mut $self_:ident, $submac:ident!( $($args:tt)* )) => (
      #[allow(unused_variables)]
      fn $name( &mut $self_, i: &[u8] ) -> $crate::lib::IResult<&[u8], &[u8], $crate::lib::Error<&[u8]>> {
        let result = $submac!(i, $($args)*);
        result
      }
  );
  // Public mutable self
  (pub $name:ident( $i:ty ) -> $o:ty, &mut $self_:ident, $submac:ident!( $($args:tt)* )) => (
      #[allow(unused_variables)]
      pub fn $name( &mut $self_, i: $i ) -> $crate::lib::IResult<$i,$o,$crate::lib::Error<$i>> {
        let result = $submac!(i, $($args)*);
        result
      }
  );
  (pub $name:ident<$i:ty,$o:ty,$e:ty>, &mut $self_:ident, $submac:ident!( $($args:tt)* )) => (
      #[allow(unused_variables)]
      pub fn $name( &mut $self_, i: $i ) -> $crate::lib::IResult<$i, $o, $e> {
        let result = $submac!(i, $($args)*);
        result
      }
  );
  (pub $name:ident<$i:ty,$o:ty>, &mut $self_:ident, $submac:ident!( $($args:tt)* )) => (
    #[allow(unused_variables)]
    pub fn $name( &mut $self_, i: $i ) -> $crate::lib::IResult<$i,$o,$crate::lib::Error<$i>> {
      let result = $submac!(i, $($args)*);
      result
    }
  );
  (pub $name:ident<$o:ty>, &mut $self_:ident, $submac:ident!( $($args:tt)* )) => (
    #[allow(unused_variables)]
    pub fn $name( &mut $self_, i: &[u8] ) -> $crate::lib::IResult<&[u8], $o, $crate::lib::Error<&[u8]>> {
      let result = $submac!(i, $($args)*);
     result
    }
  );
  (pub $name:ident, &mut $self_:ident, $submac:ident!( $($args:tt)* )) => (
    #[allow(unused_variables)]
    pub fn $name( &mut $self_, i: &[u8] ) -> $crate::lib::IResult<&[u8], &[u8], $crate::lib::Error<&[u8]>> {
      let result = $submac!(i, $($args)*);
      result
    }
  );
);

/// Used to called methods then move self back into self
#[macro_export(local_inner_macros)]
macro_rules! call_m (
  ($i:expr, $self_:ident.$method:ident) => (
    {
      $self_.$method($i)
    }
  );
  ($i:expr, $self_:ident.$method:ident, $($args:expr),* ) => (
    {
      $self_.$method($i, $($args),*)
    }
  );
);

/// emulate function currying for method calls on structs
/// `apply_m!(self.my_function, arg1, arg2, ...)` becomes `self.my_function(input, arg1, arg2, ...)`
///
/// Supports up to 6 arguments
#[macro_export(local_inner_macros)]
macro_rules! apply_m (
  ($i:expr, $self_:ident.$method:ident, $($args:expr),* ) => ( {
    $self_.$method( $i, $($args),* )
  } );
);

#[cfg(test)]
#[allow(deprecated)]
mod tests {
  // reproduce the tag_s and take_s macros, because of module import order
  macro_rules! tag_s (
    ($i:expr, $tag: expr) => (
      {
        use nom::{error::ErrorKind,Err,IResult};

        let res: IResult<_,_> = if $tag.len() > $i.len() {
          Err(Err::Error(error_position!($i, ErrorKind::Tag)))
        //} else if &$i[0..$tag.len()] == $tag {
        } else if ($i).starts_with($tag) {
          Ok((&$i[$tag.len()..], &$i[0..$tag.len()]))
        } else {
          Err(Err::Error(error_position!($i, ErrorKind::Tag)))
        };
        res
      }
    );
  );

  macro_rules! take_s (
    ($i:expr, $count:expr) => (
      {
        use nom::{IResult, Err, error::ErrorKind};

        let cnt = $count as usize;
        let res: IResult<_,_> = if $i.chars().count() < cnt {
          Err(Err::Error(error_position!($i, ErrorKind::Tag)))
        } else {
          let mut offset = $i.len();
          let mut count = 0;
          for (o, _) in $i.char_indices() {
            if count == cnt {
              offset = o;
              break;
            }
            count += 1;
          }
          Ok((&$i[offset..], &$i[..offset]))
        };
        res
      }
    );
  );

  struct Parser<'a> {
    bcd: &'a str,
  }

  impl<'a> Parser<'a> {
    pub fn new() -> Parser<'a> {
      Parser { bcd: "" }
    }

    method!(
      tag_abc<&'a str, &'a str>,
      &self,
      tag_s!("áβç")
    );
    method!(tag_bcd(&'a str) -> &'a str, &self, tag_s!("βçδ"));
    method!(pub tag_hij(&'a str) -> &'a str, &self, tag_s!("λïJ"));
    method!(pub tag_ijk<&'a str, &'a str>, &self, tag_s!("ïJƙ"));
    method!(take3<&'a str, &'a str>, &self, take_s!(3));
    method!(pub simple_call<&'a str, &'a str>, &mut self,
      call_m!(self.tag_abc)
    );
    method!(pub simple_peek<&'a str, &'a str>, &mut self,
      peek!(call_m!(self.take3))
    );
    method!(pub simple_chain<&'a str, &'a str>, &mut self,
      do_parse!(
         call_m!(self.tag_bcd_map)       >>
         last: call_m!(self.simple_peek) >>
         (last)
      )
    );

    fn tag_bcd_map(&mut self, input: &'a str) -> ::lib::IResult<&'a str, &'a str> {
      let (i, s) = self.tag_bcd(input)?;
      self.bcd = s;

      Ok((i, s))
    }


    fn tag_stuff(&mut self, input: &'a str, something: &'a str) -> ::lib::IResult<&'a str, &'a str> {
      self.bcd = something;
      self.tag_abc(input)
    }
    method!(use_apply<&'a str, &'a str>, &mut self, apply_m!(self.tag_stuff, "βçδ"));
  }

  #[test]
  fn test_method_call_abc() {
    let p = Parser::new();
    let input: &str = "áβçδèƒϱλïJƙ";
    let consumed: &str = "áβç";
    let leftover: &str = "δèƒϱλïJƙ";
    let res = p.tag_abc(input);
    match res {
      Ok((extra, output)) => {
        assert!(
          extra == leftover,
          "`Parser.tag_abc` consumed leftover input. leftover: {}",
          extra
        );
        assert!(
          output == consumed,
          "`Parser.tag_abc` doesnt return the string it consumed \
           on success. Expected `{}`, got `{}`.",
          consumed,
          output
        );
      }
      other => panic!(
        "`Parser.tag_abc` didn't succeed when it should have. \
         Got `{:?}`.",
        other
      ),
    }
  }

  #[test]
  fn test_method_call_bcd() {
    let p = Parser::new();
    let input: &str = "βçδèƒϱλïJƙ";
    let consumed: &str = "βçδ";
    let leftover: &str = "èƒϱλïJƙ";
    let res = p.tag_bcd(input);
    match res {
      Ok((extra, output)) => {
        assert!(
          extra == leftover,
          "`Parser.tag_bcd` consumed leftover input. leftover: {}",
          extra
        );
        assert!(
          output == consumed,
          "`Parser.tag_bcd` doesn't return the string it consumed \
           on success. Expected `{}`, got `{}`.",
          consumed,
          output
        );
      }
      other => panic!(
        "`Parser.tag_bcd` didn't succeed when it should have. \
         Got `{:?}`.",
        other
      ),
    }
  }

  #[test]
  fn test_method_call_hij() {
    let p = Parser::new();
    let input: &str = "λïJƙℓ₥ñôƥ9řƨ";
    let consumed: &str = "λïJ";
    let leftover: &str = "ƙℓ₥ñôƥ9řƨ";
    let res = p.tag_hij(input);
    match res {
      Ok((extra, output)) => {
        assert!(
          extra == leftover,
          "`Parser.tag_hij` consumed leftover input. leftover: {}",
          extra
        );
        assert!(
          output == consumed,
          "`Parser.tag_hij` doesn't return the string it consumed \
           on success. Expected `{}`, got `{}`.",
          consumed,
          output
        );
      }
      other => panic!(
        "`Parser.tag_hij` didn't succeed when it should have. \
         Got `{:?}`.",
        other
      ),
    }
  }

  #[test]
  fn test_method_call_ijk() {
    let p = Parser::new();
    let input: &str = "ïJƙℓ₥ñôƥ9řƨ";
    let consumed: &str = "ïJƙ";
    let leftover: &str = "ℓ₥ñôƥ9řƨ";
    let res = p.tag_ijk(input);
    match res {
      Ok((extra, output)) => {
        assert!(
          extra == leftover,
          "`Parser.tag_ijk` consumed leftover input. leftover: {}",
          extra
        );
        assert!(
          output == consumed,
          "`Parser.tag_ijk` doesn't return the string it consumed \
           on success. Expected `{}`, got `{}`.",
          consumed,
          output
        );
      }
      other => panic!(
        "`Parser.tag_ijk` didn't succeed when it should have. \
         Got `{:?}`.",
        other
      ),
    }
  }
  #[test]
  fn test_method_simple_call() {
    let mut p = Parser::new();
    let input: &str = "áβçδèƒϱλïJƙ";
    let consumed: &str = "áβç";
    let leftover: &str = "δèƒϱλïJƙ";
    let res = p.simple_call(input);
    match res {
      Ok((extra, output)) => {
        assert!(
          extra == leftover,
          "`Parser.simple_call` consumed leftover input. leftover: {}",
          extra
        );
        assert!(
          output == consumed,
          "`Parser.simple_call` doesn't return the string it consumed \
           on success. Expected `{}`, got `{}`.",
          consumed,
          output
        );
      }
      other => panic!(
        "`Parser.simple_call` didn't succeed when it should have. \
         Got `{:?}`.",
        other
      ),
    }
  }

  #[test]
  fn test_apply_m() {
    let mut p = Parser::new();
    let input: &str = "áβçδèƒϱλïJƙ";
    let consumed: &str = "áβç";
    let leftover: &str = "δèƒϱλïJƙ";
    let res = p.use_apply(input);
    match res {
      Ok((extra, output)) => {
        assert!(
          extra == leftover,
          "`Parser.use_apply` consumed leftover input. leftover: {}",
          extra
        );
        assert!(
          output == consumed,
          "`Parser.use_apply` doesn't return the string it was supposed to \
           on success. Expected `{}`, got `{}`.",
          leftover,
          output
        );
        assert!(
          p.bcd == "βçδ",
          "Parser.use_apply didn't modify the parser field correctly: {}",
          p.bcd
        );
      }
      other => panic!(
        "`Parser.use_apply` didn't succeed when it should have. \
         Got `{:?}`.",
        other
      ),
    }
  }

  #[test]
  fn test_method_call_peek() {
    let mut p = Parser::new();
    let input: &str = "ж¥ƺáβçδèƒϱλïJƙ";
    let consumed: &str = "ж¥ƺ";
    let res = p.simple_peek(input);
    match res {
      Ok((extra, output)) => {
        assert!(
          extra == input,
          "`Parser.simple_peek` consumed leftover input. leftover: {}",
          extra
        );
        assert!(
          output == consumed,
          "`Parser.simple_peek` doesn't return the string it consumed \
           on success. Expected `{}`, got `{}`.",
          consumed,
          output
        );
      }
      other => panic!(
        "`Parser.simple_peek` didn't succeed when it should have. \
         Got `{:?}`.",
        other
      ),
    }
  }

  #[test]
  fn test_method_call_chain() {
    let mut p = Parser::new();
    let input: &str = "βçδδèƒϱλïJƙℓ";
    let leftover: &str = "δèƒϱλïJƙℓ";
    let output: &str = "δèƒ";
    let res = p.simple_chain(input);
    match res {
      Ok((extra, out)) => {
        assert!(
          extra == leftover,
          "`Parser.simple_chain` consumed leftover input. leftover: {}",
          extra
        );
        assert!(
          out == output,
          "`Parser.simple_chain` doesn't return the string it was supposed to \
           on success. Expected `{}`, got `{}`.",
          output,
          out
        );
        assert!(
          p.bcd == "βçδ",
          "Parser.simple_chain didn't modify the parser field correctly: {}",
          p.bcd
        );
      }
      other => panic!(
        "`Parser.simple_chain` didn't succeed when it should have. \
         Got `{:?}`.",
        other
      ),
    }
  }
}
