
undefined4 __thiscall FUN_005c5ca0(void *this,int param_1)

{
  undefined4 *this_00;
  int iVar1;
  undefined4 uVar2;
  undefined4 *puVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00653c63;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  this_00 = (undefined4 *)FUN_00618b70(0x20);
  local_4 = 0;
  if (this_00 == (undefined4 *)0x0) {
    puVar3 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5590(this_00);
    local_4 = CONCAT31(local_4._1_3_,1);
    *this_00 = &PTR_FUN_0066c3c8;
    this_00[7] = param_1;
    puVar3 = this_00;
    if (param_1 != 0) {
      FUN_005f5600(this_00,*(undefined4 *)(param_1 + 0x18));
    }
  }
  local_4 = 0xffffffff;
  iVar1 = FUN_005c61d0(this,puVar3[6]);
  if (iVar1 == 0) {
    uVar2 = (**(code **)(*(int *)this + 0x10))(puVar3);
    ExceptionList = this_00;
    return uVar2;
  }
  ExceptionList = local_c;
  return 0;
}

