
void __thiscall FUN_0048f790(void *this,uint param_1)

{
  int *piVar1;
  int iVar2;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006359c0;
  local_c = ExceptionList;
  if (param_1 == 0x128) {
    iVar2 = 0;
    param_1 = 0x90000109;
    local_4 = 0;
    ExceptionList = &local_c;
    local_10 = this;
    piVar1 = FUN_004f3220(*(int *)((int)this + 0x20),&param_1);
    local_4 = 0xffffffff;
    FUN_00619730();
    local_10 = (void *)0x90000109;
    local_4 = 1;
    FUN_004f26d0((void *)((int)this + 0x60),&local_10);
    local_4 = 0xffffffff;
    FUN_00619730();
    if ((piVar1 != (int *)0x0) && (((byte)piVar1[9] & 0xc0) == 0x40)) {
      iVar2 = 1;
    }
    *(int *)((int)this + 0x40) = iVar2;
    if (iVar2 != 0) {
      *(undefined4 *)((int)this + 100) = 1;
      *(uint *)((int)this + 0x28) = (*(int *)((int)this + 0x20) != 1) + 1;
      FUN_0060b9d0((void *)((int)this + 0x44),0x7170,(undefined4 *)0x0,(undefined4 *)0x0,
                   (undefined4 *)0x0,(undefined4 *)0x0);
      FUN_0060b9d0((void *)((int)this + 0x50),0x7171,(undefined4 *)0x0,(undefined4 *)0x0,
                   (undefined4 *)0x0,(undefined4 *)0x0);
      *(undefined4 *)((int)this + 0x5c) = 0x74;
    }
  }
  ExceptionList = local_c;
  return;
}

