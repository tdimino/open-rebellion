
int __cdecl
FUN_00559c40(int param_1,int param_2,int param_3,int param_4,int param_5,int param_6,int param_7)

{
  int iVar1;
  int iVar2;
  
  iVar2 = 0;
  iVar1 = FUN_00559b60(param_6,param_1,param_3);
  if ((param_2 <= DAT_006bb478) && (param_7 == 0)) {
    iVar2 = DAT_006bb424;
    if ((DAT_006bb438 < param_2) && (iVar2 = DAT_006bb3f8, param_2 <= DAT_006bb434)) {
      iVar2 = DAT_006bb4b0;
    }
    iVar2 = FUN_0053e0d0(((iVar2 - param_4 * DAT_006bb3f4) - param_5 * DAT_006bb3c0) -
                         iVar1 * DAT_006bb3bc);
  }
  if (param_1 == 2) {
    iVar2 = -iVar2;
  }
  return iVar2;
}

