pub mod lexer;
pub mod parser;

fn main() {
    let mut parser = parser::Parser::new("<view class=\"container\">
    <import src=\"./footer.wxml\"></import>
    <view class=\"title\">
      <text>todos</text>
    </view>
    <view class=\"list\">
      <view class=\"list-items\">
        <input bindconfirm=\"addtodo\" placeholder=\"What needs to be done?\" value=\"{{name}}\" id=\"test\"></input>
      </view>
      <block wx:for=\"{{list}}\" wx:key=\"id\">
        <use-item iitem=\"{{item}}\" bindmyevent=\"eeevent\" bindclear=\"clear\">
        </use-item>
      </block>
      <template is=\"footer\" data=\"{{leftcount}}\"></template>
    </view>
    <button type=\"warn\" bindtap=\"toast\" style=\"margin-top:30px\">showToast</button>
    <button type=\"primary\" bindtap=\"motal\">showMotal</button>
    <button type=\"primary\" bindtap=\"navigateTo\">navigateTo</button>
  </view>");
    let res = parser.parse_all();
    println!("{:#?}", res);
}
