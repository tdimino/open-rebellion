
int __thiscall FUN_0058d440(int param_1,int *param_2,int param_3,undefined4 *param_4)

{
  uint uVar1;
  undefined4 uVar2;
  int iVar3;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
  puStack_8 = &UNK_00650028;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_4 = 0;
  *(undefined4 *)(param_1 + 8) = 0;
  if ((param_2 == (int *)0x0) || (param_3 == 0)) {
    iVar3 = 0;
  }
  else {
    iVar3 = 1;
  }
  if (iVar3 != 0) {
    uStack_4 = 0;
    uVar1 = (**(code **)(*param_2 + 4))();
    if ((uVar1 < 0x18) || (0x1b < uVar1)) {
      uVar2 = 0;
    }
    else {
      uVar2 = 1;
    }
    *(undefined4 *)(param_1 + 8) = uVar2;
    *param_4 = uVar2;
    uStack_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = pvStack_c;
  return iVar3;
}

