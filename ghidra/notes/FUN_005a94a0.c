
void __thiscall FUN_005a94a0(void *this,int param_1)

{
  undefined4 *this_00;
  undefined4 *puVar1;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00652df3;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  this_00 = (undefined4 *)FUN_00618b70(0x20);
  local_4 = 0;
  if (this_00 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5590(this_00);
    local_4 = CONCAT31(local_4._1_3_,1);
    *this_00 = &PTR_FUN_0066c3c8;
    this_00[7] = param_1;
    puVar1 = this_00;
    if (param_1 != 0) {
      FUN_005f5600(this_00,*(undefined4 *)(param_1 + 0x18));
    }
  }
  local_4 = 0xffffffff;
  (**(code **)(*(int *)((int)this + 0x9dc) + 0x10))(puVar1);
  ExceptionList = this_00;
  return;
}

