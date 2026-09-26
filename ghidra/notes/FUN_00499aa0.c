
void __thiscall FUN_00499aa0(void *this,int param_1,undefined4 param_2,uint param_3)

{
  undefined4 *puVar1;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00636958;
  local_c = ExceptionList;
  local_10 = (void *)0x0;
  if (param_1 == 0x125) {
    ExceptionList = &local_c;
    *(undefined4 *)((int)this + 0x40) = 1;
    FUN_0051cbd0(*(int *)((int)this + 0x20),param_3,(int *)&local_10);
    if (local_10 == (void *)0x0) {
      ExceptionList = local_c;
      return;
    }
    FUN_0060b9d0((void *)((int)this + 0x44),0x7048,(undefined4 *)((int)local_10 + 0x1c),
                 (undefined4 *)0x0,(undefined4 *)0x0,(undefined4 *)0x0);
    if (local_10 == (void *)0x0) {
      puVar1 = (undefined4 *)0x0;
    }
    else {
      puVar1 = (undefined4 *)((int)local_10 + 0x1c);
    }
  }
  else if (param_1 == 0x126) {
    ExceptionList = &local_c;
    *(undefined4 *)((int)this + 0x40) = 1;
    FUN_0051cbf0(*(int *)((int)this + 0x20),param_3,(int *)&local_10);
    if (local_10 == (void *)0x0) {
      ExceptionList = local_c;
      return;
    }
    FUN_0060b9d0((void *)((int)this + 0x44),0x7048,(undefined4 *)((int)local_10 + 0x1c),
                 (undefined4 *)0x0,(undefined4 *)0x0,(undefined4 *)0x0);
    if (local_10 == (void *)0x0) {
      puVar1 = (undefined4 *)0x0;
    }
    else {
      puVar1 = (undefined4 *)((int)local_10 + 0x1c);
    }
  }
  else {
    ExceptionList = &local_c;
    if (param_1 != 0x127) goto LAB_00499c14;
    ExceptionList = &local_c;
    *(undefined4 *)((int)this + 0x40) = 1;
    FUN_0051cc10(*(int *)((int)this + 0x20),param_3,(int *)&local_10);
    if (local_10 == (void *)0x0) {
      ExceptionList = local_c;
      return;
    }
    FUN_0060b9d0((void *)((int)this + 0x44),0x7048,(undefined4 *)((int)local_10 + 0x1c),
                 (undefined4 *)0x0,(undefined4 *)0x0,(undefined4 *)0x0);
    if (local_10 == (void *)0x0) {
      puVar1 = (undefined4 *)0x0;
    }
    else {
      puVar1 = (undefined4 *)((int)local_10 + 0x1c);
    }
  }
  FUN_0060b9d0((void *)((int)this + 0x50),0x7049,puVar1,(undefined4 *)0x0,(undefined4 *)0x0,
               (undefined4 *)0x0);
LAB_00499c14:
  if ((local_10 != (void *)0x0) && (*(int *)((int)this + 0x40) != 0)) {
    puVar1 = (undefined4 *)FUN_00402e40(local_10,&param_3);
    local_4 = 0;
    FUN_004f26d0((void *)((int)this + 0x60),puVar1);
    local_4 = 0xffffffff;
    FUN_00619730();
    *(undefined4 *)((int)this + 0x28) = 4;
    *(undefined2 *)((int)this + 0x30) = 0x459;
    *(undefined2 *)((int)this + 0x2e) = 0x3f9;
  }
  ExceptionList = local_c;
  return;
}

