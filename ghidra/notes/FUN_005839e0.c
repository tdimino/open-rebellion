
int __thiscall FUN_005839e0(void *this,void *param_1)

{
  void *this_00;
  void *this_01;
  int *piVar1;
  int *piVar2;
  int iVar3;
  int local_14;
  undefined1 local_10 [4];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064f1b8;
  local_c = ExceptionList;
  local_14 = 0;
  ExceptionList = &local_c;
  this_01 = (void *)FUN_00583c40((int)this);
  this_00 = param_1;
  for (; this_01 != (void *)0x0; this_01 = *(void **)((int)this_01 + 8)) {
    piVar1 = FUN_0052fd10(this_00,local_10);
    local_4 = 0;
    piVar2 = FUN_0052fd10(this_01,&param_1);
    if ((*piVar2 == *piVar1) && (*(int *)((int)this_01 + 0x14) == *(int *)((int)this_00 + 0x14))) {
      iVar3 = 1;
    }
    else {
      iVar3 = 0;
    }
    local_14 = local_14 + iVar3;
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return local_14;
}

