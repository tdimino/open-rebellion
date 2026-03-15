
bool __thiscall FUN_00529360(void *this,void *param_1,int *param_2)

{
  void *this_00;
  int *piVar1;
  int iVar2;
  undefined *puVar3;
  void *this_01;
  int *piVar4;
  bool bVar5;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_00 = param_1;
  puStack_8 = &LAB_00644778;
  local_c = ExceptionList;
  bVar5 = true;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_1,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  piVar1 = param_2;
  *param_2 = 0;
  if (*(int *)((int)this + 0x58) == 0) {
    local_14 = 0xa0;
    local_10 = 1;
    local_4 = 1;
    FUN_00520580(this_00,&local_14);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  if ((((byte)*(undefined4 *)((int)this + 0x24) & 0xf) == 1) && (*(int *)((int)this_00 + 4) == -1))
  {
    iVar2 = FUN_00539fd0(this,0);
    puVar3 = FUN_00528820(iVar2);
    this_01 = (void *)thunk_FUN_005f5080((int)puVar3);
    bVar5 = this_01 != (void *)0x0;
    if (this_01 != (void *)0x0) {
      piVar4 = (int *)FUN_00403040(this_01,&param_1);
      local_4 = 2;
      iVar2 = FUN_00529120(this,piVar4,this_00,piVar1);
      if ((iVar2 == 0) || (!bVar5)) {
        bVar5 = false;
      }
      else {
        bVar5 = true;
      }
      local_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  ExceptionList = local_c;
  return bVar5;
}

