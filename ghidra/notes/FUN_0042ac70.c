
bool __thiscall FUN_0042ac70(void *this,int param_1)

{
  uint uVar1;
  int iVar2;
  undefined4 *puVar3;
  uint uVar4;
  undefined4 *this_00;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062c5bb;
  local_c = ExceptionList;
  uVar1 = *(uint *)(param_1 + 0x24);
  ExceptionList = &local_c;
  iVar2 = FUN_005f5500((void *)((int)this + 0x478),uVar1);
  if (iVar2 == 0) {
    puVar3 = (undefined4 *)FUN_00618b70(0x1c);
    this_00 = (undefined4 *)0x0;
    local_4 = 0;
    if (puVar3 != (undefined4 *)0x0) {
      this_00 = FUN_005f5590(puVar3);
    }
    local_4 = 0xffffffff;
    FUN_005f5600(this_00,uVar1);
    uVar4 = FUN_005f5440((void *)((int)this + 0x478),this_00);
    ExceptionList = local_c;
    return uVar4 == uVar1;
  }
  ExceptionList = local_c;
  return true;
}

