
int __cdecl FUN_0056c990(int param_1,uint param_2,void *param_3,undefined4 *param_4)

{
  void *this;
  bool bVar1;
  int *piVar2;
  void *this_00;
  uint *puVar3;
  undefined3 extraout_var;
  int iVar4;
  int iVar5;
  void *local_20;
  int local_1c;
  int local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this = param_3;
  puStack_8 = &LAB_0064bb20;
  local_c = ExceptionList;
  iVar5 = 1;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_3,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  param_3 = (void *)0x0;
  *param_4 = 0;
  if (*(int *)((int)this + 4) == -1) {
    piVar2 = (int *)FUN_00402e40(*(void **)(param_2 + 0x14),&param_2);
    local_4 = 1;
    iVar5 = FUN_0056c460(piVar2,&param_3);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  this_00 = (void *)thunk_FUN_005f5060(param_1);
  while (((this_00 != (void *)0x0 && (iVar5 != 0)) && (*(int *)((int)this + 4) == -1))) {
    puVar3 = FUN_00559950(this_00,&param_2);
    local_4 = 2;
    bVar1 = FUN_00582dd0(puVar3,&local_20);
    local_4 = 0xffffffff;
    FUN_00619730();
    iVar5 = 0;
    if (CONCAT31(extraout_var,bVar1) != 0) {
      iVar4 = FUN_00593c30(local_20,(int)param_3,&local_1c);
      iVar5 = 0;
      if (((iVar4 != 0) && (iVar5 = FUN_00593c70(local_20,(int)param_3,&local_18), iVar5 != 0)) &&
         ((local_1c == 0 && (local_18 == 0)))) {
        local_14 = 0x40;
        local_10 = 4;
        local_4 = 3;
        FUN_00520580(this,&local_14);
        local_4 = 0xffffffff;
        FUN_00619730();
        *param_4 = 0xffffffff;
      }
    }
    this_00 = *(void **)((int)this_00 + 0x10);
  }
  ExceptionList = local_c;
  return iVar5;
}

