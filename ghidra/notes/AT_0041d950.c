
/* lpfn parameter of SetWindowsHookExA
    */

undefined4 lpfn_0041d950(int param_1,WPARAM param_2,LPARAM param_3)

{
  bool bVar1;
  LRESULT LVar2;
  
  bVar1 = false;
  if (param_1 == 0) {
    switch(param_2) {
    case 0x201:
    case 0x204:
      FUN_0043a200();
      bVar1 = true;
      break;
    case 0x202:
    case 0x205:
      bVar1 = true;
      break;
    default:
      bVar1 = false;
    }
  }
  if ((!bVar1) && (LVar2 = CallNextHookEx(DAT_006b14b0,param_1,param_2,param_3), LVar2 == 0)) {
    return 0;
  }
  return 1;
}

