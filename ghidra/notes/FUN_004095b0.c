
int __thiscall FUN_004095b0(void *this,int *param_1)

{
  int *piVar1;
  bool bVar2;
  int iVar3;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  int iVar4;
  
  iVar4 = 1;
  FUN_005f4d70(param_1);
  piVar1 = (int *)((int)this + 4);
  FUN_005f4db0(param_1,piVar1);
  FUN_005f4db0(param_1,(int *)((int)this + 8));
  FUN_005f4970((void *)((int)this + 0xc),param_1);
  FUN_005f4970((void *)((int)this + 0x10),param_1);
  iVar3 = *(int *)((int)this + 8);
  if (iVar3 != 2) {
    if (iVar3 == 0x14) {
      iVar3 = FUN_00597530();
      iVar4 = 0;
      if (*(int *)(iVar3 + 0x694) != 0) {
        iVar3 = FUN_0051ebb0();
        iVar4 = 0;
        if (iVar3 != 0) {
          bVar2 = FUN_00401290(param_1);
          iVar4 = 0;
          if (CONCAT31(extraout_var,bVar2) != 0) {
            bVar2 = FUN_004098d0(param_1);
            iVar4 = CONCAT31(extraout_var_00,bVar2);
          }
        }
      }
    }
    *piVar1 = iVar4;
    return iVar4;
  }
  FUN_005f4d70(param_1);
  if (DAT_006b1218 != (void *)0x0) {
    FUN_0041de70(DAT_006b1218,param_1);
  }
  FUN_005f4d70(param_1);
  FUN_00401250(param_1);
  FUN_005f4d70(param_1);
  *piVar1 = 1;
  return 1;
}

