
int __thiscall FUN_004f9860(void *this,int param_1,void *param_2)

{
  int iVar1;
  uint uVar2;
  int iVar3;
  void *unaff_EBX;
  undefined4 local_14;
  undefined4 local_10;
  void *pvStack_c;
  int *piStack_8;
  undefined4 local_4;
  
  piStack_8 = (int *)&LAB_0063ff20;
  pvStack_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_2,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  iVar1 = (**(code **)(*(int *)this + 0x58))(0,*(undefined4 *)(param_1 + 0xc),param_2);
  if (iVar1 != 0) {
    if (*(int *)((int)param_2 + 4) == -1) {
      uVar2 = *(uint *)((int)this + 0x24) >> 6 & 3;
      if (uVar2 == 1) {
        iVar3 = 1;
      }
      else if (uVar2 == 2) {
        iVar3 = 2;
      }
      else {
        iVar3 = 0;
      }
      if ((iVar3 == 0) || (iVar3 != *piStack_8)) {
        local_10 = 1;
        FUN_00520580(param_2,(undefined4 *)&stack0xffffffe0);
        local_10 = 0xffffffff;
        FUN_00619730();
      }
    }
    if (iVar1 != 0) {
      if (((((*(int *)((int)param_2 + 4) == -1) && ((*(byte *)((int)this + 0x50) & 4) == 0)) &&
           (iVar3 = piStack_8[3], iVar3 != 0x203)) && ((iVar3 != 0x215 && (iVar3 != 0x204)))) &&
         (iVar3 != 0x241)) {
        local_10 = 2;
        FUN_00520580(param_2,(undefined4 *)&stack0xffffffe0);
        local_10 = 0xffffffff;
        FUN_00619730();
      }
      if (iVar1 != 0) {
        if ((*(int *)((int)param_2 + 4) == -1) && ((*(byte *)((int)this + 0x50) & 8) != 0)) {
          local_10 = 3;
          FUN_00520580(param_2,(undefined4 *)&stack0xffffffe0);
          local_10 = 0xffffffff;
          FUN_00619730();
        }
        if (iVar1 != 0) {
          if ((*(int *)((int)param_2 + 4) == -1) && ((*(byte *)((int)this + 0x50) & 0x10) != 0)) {
            local_10 = 4;
            FUN_00520580(param_2,(undefined4 *)&stack0xffffffe0);
            local_10 = 0xffffffff;
            FUN_00619730();
          }
          if (iVar1 != 0) {
            ExceptionList = unaff_EBX;
            return iVar1;
          }
        }
      }
    }
  }
  local_10 = 5;
  FUN_00520580(param_2,(undefined4 *)&stack0xffffffe0);
  local_10 = 0xffffffff;
  FUN_00619730();
  ExceptionList = unaff_EBX;
  return iVar1;
}

