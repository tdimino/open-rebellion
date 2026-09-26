
int __thiscall FUN_00529120(void *this,int *param_1,void *param_2,int *param_3)

{
  int iVar1;
  undefined *puVar2;
  void *this_00;
  int *piVar3;
  int *piVar4;
  int iVar5;
  uint *puVar6;
  uint uVar7;
  void *this_01;
  int local_20;
  int local_1c [4];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_01 = param_2;
  puStack_8 = &LAB_00644750;
  local_c = ExceptionList;
  local_1c[0] = 1;
  local_1c[1] = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_2,local_1c);
  local_4 = 0xffffffff;
  FUN_00619730();
  *param_3 = 0;
  if (*(int *)((int)this + 0x58) == 0) {
    local_1c[0] = 0xa0;
    local_1c[1] = 1;
    local_4 = 1;
    FUN_00520580(this_01,local_1c);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  if (((byte)*(undefined4 *)((int)this + 0x24) & 0xf) != 1) {
    ExceptionList = local_c;
    return 1;
  }
  local_1c[0] = 1;
  if (*(int *)((int)this_01 + 4) != -1) {
    ExceptionList = local_c;
    return 1;
  }
  iVar1 = FUN_00539fd0(this,0);
  puVar2 = FUN_00528820(iVar1);
  this_00 = (void *)thunk_FUN_005f5060((int)puVar2);
  piVar4 = param_1;
  if (this_00 == (void *)0x0) {
LAB_00529231:
    local_1c[2] = 0xa0;
    local_1c[3] = 5;
    local_4 = 2;
    FUN_00520580(this_01,local_1c + 2);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  else {
    do {
      piVar3 = FUN_00403040(this_00,&param_1);
      iVar1 = *piVar4;
      iVar5 = *piVar3;
      FUN_00619730();
      if (iVar1 == iVar5) break;
      this_00 = *(void **)((int)this_00 + 0x10);
      local_1c[0] = local_1c[0] + 1;
    } while (this_00 != (void *)0x0);
    this_01 = param_2;
    if (this_00 == (void *)0x0) goto LAB_00529231;
  }
  if (*(int *)((int)this_01 + 4) != -1) {
    ExceptionList = local_c;
    return 1;
  }
  if (this_00 != (void *)0x0) {
    piVar4 = FUN_00527c00(this,&param_2);
    local_4 = 3;
    iVar1 = FUN_004f26f0(piVar4);
    local_4 = 0xffffffff;
    FUN_00619730();
    if (iVar1 != 0) {
      local_20 = 1;
      goto LAB_005292b9;
    }
  }
  local_20 = 0;
LAB_005292b9:
  iVar1 = local_20;
  if (this_00 != (void *)0x0) {
    piVar4 = FUN_00527c00(this,&param_2);
    local_4 = 4;
    iVar5 = FUN_004f26f0(piVar4);
    local_4 = 0xffffffff;
    FUN_00619730();
    if (iVar5 != 0) {
      puVar6 = FUN_00527c00(this,&param_2);
      local_4 = 5;
      uVar7 = FUN_00528d30(this,puVar6,local_1c[0],1,this_01,param_3);
      if ((uVar7 == 0) || (local_20 = 1, iVar1 == 0)) {
        local_20 = 0;
      }
      local_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  ExceptionList = local_c;
  return local_20;
}

