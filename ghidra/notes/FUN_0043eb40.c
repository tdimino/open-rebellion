
void __thiscall FUN_0043eb40(void *this,int *param_1)

{
  void *pvVar1;
  void *unaff_ESI;
  void *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062e358;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  local_10 = this;
  pvVar1 = FUN_0043f110(this,&local_10);
  local_4 = 0;
  (**(code **)(*param_1 + 0x16c))((undefined4 *)((int)this + 0x20),pvVar1);
  pvStack_c = (void *)0xffffffff;
  FUN_00619730();
  if (*(int *)((int)this + 0xc) == 0) {
    FUN_0041dc80(*(int *)((int)this + 0x18),(undefined4 *)((int)this + 0x20));
  }
  ExceptionList = unaff_ESI;
  return;
}

