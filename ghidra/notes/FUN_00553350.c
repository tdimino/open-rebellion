
int __cdecl FUN_00553350(uint param_1,void *param_2,int param_3,uint param_4)

{
  uint uVar1;
  void *this;
  bool bVar2;
  int iVar3;
  void *this_00;
  uint *puVar4;
  undefined3 extraout_var;
  int iVar5;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  uVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00648fb8;
  local_c = ExceptionList;
  iVar5 = 1;
  ExceptionList = &local_c;
  FUN_00513120(param_1);
  this = param_2;
  do {
    if (*(int *)(uVar1 + 0x1c) == 0) {
      ExceptionList = local_c;
      return iVar5;
    }
    if ((param_3 == 0) ||
       (iVar3 = FUN_0052bed0(uVar1), param_4 == (*(uint *)(iVar3 + 0x24) >> 6 & 3))) {
      puVar4 = &param_1;
      this_00 = (void *)FUN_0052bed0(uVar1);
      puVar4 = FUN_004025b0(this_00,puVar4);
      local_4 = 0;
      bVar2 = FUN_004f44b0(this,puVar4,0);
      iVar5 = CONCAT31(extraout_var,bVar2);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
    FUN_005130d0(uVar1);
  } while (iVar5 != 0);
  ExceptionList = local_c;
  return 0;
}

