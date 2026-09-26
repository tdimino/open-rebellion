
int __thiscall FUN_00519c20(void *this,int param_1,uint param_2)

{
  void *this_00;
  uint *puVar1;
  int *piVar2;
  int iVar3;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642d30;
  local_c = ExceptionList;
  iVar3 = 1;
  ExceptionList = &local_c;
  FUN_004fd450(&local_28,param_2);
  local_4 = 0;
  local_24 = 1;
  local_20 = 2;
  for (this_00 = (void *)FUN_0051cad0(0x80); this_00 != (void *)0x0;
      this_00 = *(void **)((int)this_00 + 0x10)) {
    if (*(int *)((int)this_00 + 0x44) <= param_1) {
      puVar1 = (uint *)FUN_00402e40(this_00,&param_2);
      local_4._0_1_ = 1;
      piVar2 = FUN_004f7d50(this,puVar1,3,&local_28,1,1);
      if ((piVar2 == (int *)0x0) || (iVar3 == 0)) {
        iVar3 = 0;
      }
      else {
        iVar3 = 1;
      }
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
    }
  }
  local_4 = 0xffffffff;
  FUN_004fd4d0(&local_28);
  ExceptionList = local_c;
  return iVar3;
}

