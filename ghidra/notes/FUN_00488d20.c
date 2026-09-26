
void __thiscall FUN_00488d20(void *this,uint *param_1,int param_2)

{
  bool bVar1;
  void *this_00;
  int *piVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00634c3b;
  local_c = ExceptionList;
  if ((*param_1 >> 0x18 < 0x90) || (0x97 < *param_1 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  ExceptionList = &local_c;
  FUN_00619730();
  if (bVar1) {
    this_00 = (void *)FUN_00618b70(0x7c);
    local_4 = 0;
    if (this_00 == (void *)0x0) {
      piVar2 = (int *)0x0;
    }
    else {
      piVar2 = FUN_0049be60(this_00,param_1,*(int *)this);
    }
    local_4 = 0xffffffff;
    if (piVar2 != (int *)0x0) {
      piVar2[0x14] = param_2;
    }
    FUN_0048a060(this,piVar2);
  }
  ExceptionList = local_c;
  return;
}

