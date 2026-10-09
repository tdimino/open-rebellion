
/* lpfn parameter of SetWindowsHookExA
    */

bool lpfn_0041d8f0(int param_1,WPARAM param_2,uint param_3)

{
  bool bVar1;
  
  bVar1 = DAT_006b14b4 == 0;
  if ((((-1 < param_1) && (DAT_006b14b4 == 0)) && (param_2 == 0x1b)) &&
     ((param_3 & 0x80000000) != 0)) {
    FUN_0043a200();
  }
  CallNextHookEx(DAT_006b14ac,param_1,param_2,param_3);
  return bVar1;
}

