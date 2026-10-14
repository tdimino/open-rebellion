
int __thiscall FUN_00510d70(int *param_1,uint *param_2,uint *param_3,void *param_4)

{
  int iVar1;
  uint uVar2;
  int iVar3;
  
  iVar3 = 1;
  if (DAT_006b90e0 != 0) {
    if (((*param_2 ^ *param_3) & 3) != 0) {
      if (((*param_2 & 0x30) == 0) && ((*param_3 & 0x30) == 0)) {
        iVar3 = 1;
      }
      else {
        iVar3 = 0;
      }
      iVar3 = FUN_0050a970(param_1,iVar3,param_4);
      iVar1 = FUN_004f6f40(param_1,*param_3 & 3,param_4);
      if ((iVar1 == 0) || (iVar3 == 0)) {
        iVar3 = 0;
      }
      else {
        iVar3 = 1;
      }
    }
    uVar2 = FUN_0053f950(0x31b,param_1,*param_2,*param_3,param_4);
    if ((uVar2 != 0) && (iVar3 != 0)) {
      return 1;
    }
    iVar3 = 0;
  }
  return iVar3;
}

