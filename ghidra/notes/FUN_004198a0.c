
void __thiscall FUN_004198a0(void *this,undefined4 param_1,int param_2)

{
  int iVar1;
  int *piVar2;
  void *local_14;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062b200;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  if (param_2 == *(int *)((int)this + 0x140)) {
    ExceptionList = &local_c;
    *(uint *)((int)this + 4) = *(uint *)((int)this + 4) | 0x2000000;
  }
  iVar1 = FUN_0051cbd0(*(int *)this,param_2,(int *)&local_14);
  if (iVar1 != 0) {
    if (*(int *)this == 1) {
      local_4 = 0;
      piVar2 = (int *)FUN_00402e40(local_14,&local_10);
      iVar1 = *piVar2;
      FUN_00619730();
      local_4 = 0xffffffff;
      FUN_00619730();
      if (iVar1 != 0x1400004a) goto LAB_0041998d;
    }
    else {
      local_4 = 1;
      piVar2 = (int *)FUN_00402e40(local_14,&local_10);
      iVar1 = *piVar2;
      FUN_00619730();
      local_4 = 0xffffffff;
      FUN_00619730();
      if (iVar1 != 0x14000082) goto LAB_0041998d;
    }
    *(uint *)((int)this + 4) = *(uint *)((int)this + 4) | 0x1000000;
  }
LAB_0041998d:
  FUN_0042e210((void *)((int)this + 0x138),param_2,*(int *)this);
  ExceptionList = local_c;
  return;
}

