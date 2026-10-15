
void __thiscall FUN_004f11f0(int param_1,undefined4 param_2,undefined4 param_3)

{
  int iVar1;
  undefined4 uVar2;
  int iVar3;
  bool bVar4;
  undefined4 uVar5;
  
  iVar1 = FUN_00535e40(param_2,param_3);
  iVar3 = 0;
  bVar4 = false;
  if (iVar1 != 0) {
    iVar3 = *(int *)(param_1 + 0x2c);
    bVar4 = iVar3 != 0;
  }
  if (bVar4) {
    uVar2 = *(undefined4 *)(iVar3 + 0x98);
    iVar3 = FUN_004ee950(*(undefined4 *)(iVar3 + 0x9c),param_3);
    if (iVar3 != 0) {
      uVar5 = param_3;
      uVar2 = FUN_0053e2f0(uVar2,param_3);
      iVar3 = FUN_004ee9c0(uVar2,uVar5);
      if ((((((iVar3 != 0) && (iVar3 = FUN_004eee60(param_3), iVar3 != 0)) &&
            (iVar3 = FUN_004eee40(param_3), iVar3 != 0)) &&
           ((iVar3 = FUN_004eecb0(param_3), iVar3 != 0 &&
            (iVar3 = FUN_004eecf0(param_3), iVar3 != 0)))) &&
          ((iVar3 = FUN_004eed30(param_3), iVar3 != 0 &&
           ((iVar3 = FUN_004eed50(param_3), iVar3 != 0 &&
            (iVar3 = FUN_004eed70(param_3), iVar3 != 0)))))) &&
         ((iVar3 = FUN_004eed90(param_3), iVar3 != 0 && (iVar3 = FUN_004eede0(param_3), iVar3 != 0))
         )) {
        FUN_004eee20(param_3);
      }
    }
  }
  return;
}

