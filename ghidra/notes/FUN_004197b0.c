
void __thiscall FUN_004197b0(void *this,undefined4 param_1,int param_2)

{
  int iVar1;
  int *piVar2;
  void *local_14;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062b1d8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  if (param_2 == *(int *)((int)this + 0x148)) {
    ExceptionList = &local_c;
    *(uint *)((int)this + 4) = *(uint *)((int)this + 4) | 0x8000000;
  }
  iVar1 = FUN_0051cc10(*(int *)this,param_2,(int *)&local_14);
  if (iVar1 != 0) {
    local_4 = 0;
    piVar2 = (int *)FUN_00402e40(local_14,&local_10);
    iVar1 = *piVar2;
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
    if (iVar1 == 0x25000004) {
      *(uint *)((int)this + 4) = *(uint *)((int)this + 4) | 0x1000000;
    }
  }
  FUN_0042e3b0((void *)((int)this + 0x138),param_2);
  ExceptionList = local_c;
  return;
}

