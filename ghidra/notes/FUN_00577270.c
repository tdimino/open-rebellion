
int __thiscall
FUN_00577270(void *this,uint param_1,uint param_2,int param_3,uint *param_4,void *param_5)

{
  bool bVar1;
  int iVar2;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0064cc90;
  local_c = ExceptionList;
  iVar2 = 1;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_5,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  if (param_3 == 0x240) {
    local_14 = 0x30;
    local_10 = 0x40;
    if ((*param_4 < 0x30) || (0x40 < param_4[1])) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      bVar1 = true;
      goto LAB_00577317;
    }
  }
  bVar1 = false;
LAB_00577317:
  if (param_3 == 0x241) {
    local_14 = 0x30;
    local_10 = 0x40;
    FUN_00619730();
  }
  if ((((*(int *)((int)param_5 + 4) == -1) &&
       (iVar2 = FUN_00553410((int)this,param_1,param_2,1,1,param_5), iVar2 != 0)) &&
      (*(int *)((int)param_5 + 4) != -1)) && (bVar1)) {
    local_10 = 0xffffffff;
    local_14 = 1;
    local_4 = 1;
    FUN_00520580(param_5,&local_14);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return iVar2;
}

