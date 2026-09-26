
void __thiscall FUN_0040fd20(void *this,undefined4 param_1)

{
  int iVar1;
  void *this_00;
  int *piVar2;
  undefined4 local_28;
  int local_24;
  uint local_14;
  undefined4 *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062a457;
  local_c = ExceptionList;
  iVar1 = *(int *)((int)this + 0x120);
  ExceptionList = &local_c;
  this_00 = (void *)FUN_00618b70(0x34);
  if (this_00 == (void *)0x0) {
    FUN_00619730();
  }
  local_4 = 0;
  if (this_00 == (void *)0x0) {
    piVar2 = (int *)0x0;
  }
  else {
    piVar2 = FUN_00420d70(this_00,*(undefined4 *)((int)this + 0x130),2 - (uint)(iVar1 != 1),
                          *(undefined4 *)((int)this + 0x124),*(undefined4 *)((int)this + 0x128),
                          param_1);
  }
  local_4 = 0xffffffff;
  if (piVar2 != (int *)0x0) {
    FUN_0051c300(&local_28);
    local_4 = 1;
    FUN_0051c410(&local_28,piVar2);
    if (local_24 != 0) {
      FUN_005970d0(local_10,local_14,1);
      FUN_00597170();
    }
    (**(code **)*piVar2)(1);
    local_4 = 0xffffffff;
    FUN_0051c350(&local_28);
  }
  ExceptionList = local_c;
  return;
}

