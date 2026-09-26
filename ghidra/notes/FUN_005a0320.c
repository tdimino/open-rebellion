
void __thiscall FUN_005a0320(void *this,uint param_1)

{
  uint uVar1;
  int iVar2;
  undefined4 *puVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00651cfb;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  uVar1 = FUN_005a02c0(this,param_1);
  if (uVar1 < 4) {
    *(undefined4 *)((int)this + uVar1 * 4 + 0x154) = 0;
    iVar2 = (**(code **)(*DAT_006bc464 + 8))();
    if (8 < iVar2) {
      iVar2 = (**(code **)(*DAT_006bc464 + 8))();
      if (iVar2 < 0xd) {
        iVar2 = (**(code **)(*DAT_006bc464 + 8))();
        if (iVar2 - 9U == uVar1) {
          puVar3 = (undefined4 *)FUN_00618b70(0x54);
          uStack_4 = 0;
          if (puVar3 == (undefined4 *)0x0) {
            puVar3 = (undefined4 *)0x0;
          }
          else {
            puVar3 = FUN_005a0c80(puVar3);
          }
          uStack_4 = 0xffffffff;
          FUN_0059f350(puVar3);
        }
      }
    }
    FUN_0059fca0(this);
  }
  ExceptionList = local_c;
  return;
}

