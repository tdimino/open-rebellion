
int __cdecl
FUN_0053fd60(uint param_1,void *param_2,undefined4 param_3,undefined4 param_4,undefined4 param_5,
            undefined4 param_6,int param_7)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  
  if ((param_1 < 0x600) || (0x621 < param_1)) {
    iVar2 = 0;
  }
  else {
    iVar2 = 1;
  }
  if (iVar2 != 0) {
    bVar1 = FUN_005400f0(param_1,param_2,param_7,(int *)&param_1);
    iVar2 = CONCAT31(extraout_var,bVar1);
    if ((iVar2 != 0) && (param_1 != 0)) {
      *(undefined4 *)(param_1 + 0x24) = param_3;
      *(undefined4 *)(param_1 + 0x28) = param_4;
      *(undefined4 *)(param_1 + 0x2c) = param_5;
      *(undefined4 *)(param_1 + 0x30) = param_6;
      FUN_004fd3b0(param_1);
    }
  }
  return iVar2;
}

