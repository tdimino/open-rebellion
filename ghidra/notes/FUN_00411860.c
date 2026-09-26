
void * __thiscall FUN_00411860(void *this,void *param_1)

{
  void *pvVar1;
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062a6e8;
  local_c = ExceptionList;
  if (this != param_1) {
    ExceptionList = &local_c;
    pvVar1 = FUN_00408d90(param_1,local_18);
    local_4 = 0;
    FUN_005f3090((void *)((int)this + 4),(int)pvVar1);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_18);
    *(undefined4 *)((int)this + 0x10) = *(undefined4 *)((int)param_1 + 0x10);
    *(undefined4 *)((int)this + 0x14) = *(undefined4 *)((int)param_1 + 0x14);
    *(undefined4 *)((int)this + 0x18) = *(undefined4 *)((int)param_1 + 0x18);
    *(undefined4 *)((int)this + 0x1c) = *(undefined4 *)((int)param_1 + 0x1c);
    *(undefined4 *)((int)this + 0x20) = *(undefined4 *)((int)param_1 + 0x20);
    *(undefined4 *)((int)this + 0x24) = *(undefined4 *)((int)param_1 + 0x24);
  }
  ExceptionList = local_c;
  return this;
}

