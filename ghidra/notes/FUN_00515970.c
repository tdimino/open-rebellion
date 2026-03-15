
bool __cdecl FUN_00515970(uint *param_1,int param_2,void *param_3)

{
  bool bVar1;
  int *piVar2;
  uint uVar3;
  void *this;
  undefined3 extraout_var;
  bool bVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642728;
  local_c = ExceptionList;
  bVar4 = true;
  ExceptionList = &local_c;
  piVar2 = (int *)FUN_00504dc0(param_1);
  if ((piVar2 != (int *)0x0) && ((*(byte *)(piVar2 + 0x14) & 8) == 0)) {
    local_4 = 0;
    uVar3 = (**(code **)(*piVar2 + 4))();
    if ((uVar3 < 0x30) || (0x3f < uVar3)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (bVar1) {
      this = (void *)thunk_FUN_00506e40();
      bVar4 = this != (void *)0x0;
      if (this != (void *)0x0) {
        bVar1 = FUN_00545df0(this,param_1,param_2,param_3);
        if ((CONCAT31(extraout_var,bVar1) != 0) && (bVar4)) {
          ExceptionList = local_c;
          return true;
        }
        bVar4 = false;
      }
    }
  }
  ExceptionList = local_c;
  return bVar4;
}

