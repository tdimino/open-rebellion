
bool __cdecl FUN_00517de0(int param_1,undefined4 param_2,uint *param_3,void *param_4)

{
  bool bVar1;
  int iVar2;
  int *piVar3;
  uint uVar4;
  undefined3 extraout_var;
  bool bVar5;
  uint local_28 [7];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642a00;
  local_c = ExceptionList;
  bVar5 = true;
  if ((*(byte *)(param_1 + 0x50) & 0x10) != 0) {
    ExceptionList = &local_c;
    FUN_00562ae0(local_28);
    local_4 = 0;
    iVar2 = FUN_0055e150(local_28);
    bVar5 = iVar2 != 0;
    piVar3 = (int *)FUN_00504dc0(param_3);
    if (piVar3 != (int *)0x0) {
      local_4._0_1_ = 1;
      uVar4 = (**(code **)(*piVar3 + 4))();
      if ((uVar4 < 0x90) || (0x97 < uVar4)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
      if (bVar1) {
        bVar1 = FUN_00518ac0(piVar3,local_28,param_4);
        if ((CONCAT31(extraout_var,bVar1) == 0) || (!bVar5)) {
          bVar5 = false;
        }
        else {
          bVar5 = true;
        }
      }
    }
    local_4 = 0xffffffff;
    FUN_00562b30();
  }
  ExceptionList = local_c;
  return bVar5;
}

