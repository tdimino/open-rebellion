
void FUN_0058d540(int *param_1,undefined4 param_2,undefined4 *param_3)

{
  int *piVar1;
  undefined4 uVar2;
  int iVar3;
  int iVar4;
  int iStack_8;
  int iStack_4;
  
  piVar1 = param_1;
  *param_3 = 0;
  iStack_8 = FUN_005006e0();
  param_3 = (undefined4 *)FUN_0053a6e0();
  iVar3 = (**(code **)(*piVar1 + 0x204))();
  iVar4 = (**(code **)(*piVar1 + 0x1e0))();
  param_1 = (int *)(iVar4 - iVar3);
  iVar3 = (**(code **)(*piVar1 + 0x274))();
  iStack_4 = (**(code **)(*piVar1 + 0x248))();
  iStack_4 = iStack_4 - iVar3;
  func_0x0055e9a0(iStack_8,param_1,&iStack_8,&param_1);
  func_0x0055e9a0(param_3,param_1,&param_3,&param_1);
  func_0x0055e9a0(param_3,iStack_4,&param_3,&iStack_4);
  FUN_00501ba0(iStack_8);
  FUN_00501bc0(param_3);
  iVar3 = (**(code **)(*piVar1 + 0x204))();
  iVar4 = (**(code **)(*piVar1 + 0x1e0))();
  func_0x00501be0((iVar4 - iVar3) - (int)param_1);
  iVar3 = (**(code **)(*piVar1 + 0x274))();
  iVar4 = (**(code **)(*piVar1 + 0x248))();
  uVar2 = param_3;
  iVar3 = (iVar4 - iVar3) - iStack_8;
  iVar4 = FUN_00501ae0(iVar3,0xf,param_3);
  if (iVar4 != 0) {
    func_0x005018c0(iVar3,0xf,uVar2);
  }
  return;
}

