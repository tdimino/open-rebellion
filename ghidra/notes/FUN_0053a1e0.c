
uint __thiscall FUN_0053a1e0(void *this,int *param_1)

{
  int *piVar1;
  bool bVar2;
  undefined4 *puVar3;
  undefined3 extraout_var;
  int *this_00;
  uint uVar4;
  uint uVar5;
  int local_18;
  int local_14;
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064629f;
  local_c = ExceptionList;
  uVar5 = 1;
  ExceptionList = &local_c;
  FUN_005f4d90(param_1,&local_14);
  if (local_14 != 0) {
    puVar3 = (undefined4 *)FUN_00618b70(0x10);
    if (puVar3 == (undefined4 *)0x0) {
      FUN_00619730();
    }
    local_4 = 0;
    if (puVar3 == (undefined4 *)0x0) {
      puVar3 = (undefined4 *)0x0;
    }
    else {
      puVar3 = FUN_00584150(puVar3);
    }
    local_4 = 0xffffffff;
    *(undefined4 **)((int)this + 0x28) = puVar3;
    uVar5 = 0;
    if ((puVar3 != (undefined4 *)0x0) && (uVar5 = 0, puVar3[3] != 0)) {
      uVar5 = FUN_005841a0(puVar3,piVar1);
    }
    param_1 = (int *)0x0;
    if (((uVar5 != 0) && (FUN_005f4d90(piVar1,&param_1), uVar5 != 0)) && (param_1 != (int *)0x0)) {
      bVar2 = FUN_005f4d40(piVar1);
      uVar5 = CONCAT31(extraout_var,bVar2);
    }
    local_18 = 0;
    uVar4 = uVar5 & 0 < (int)param_1;
    while (uVar4 != 0) {
      FUN_005f4d90(piVar1,&local_10);
      this_00 = (int *)FUN_00540500(local_10);
      uVar5 = 0;
      if (((this_00 != (int *)0x0) && (uVar5 = (**(code **)(*this_00 + 0x14))(piVar1), uVar5 != 0))
         && (FUN_0060f5d0(this_00,this), uVar5 != 0)) {
        uVar4 = FUN_005f5440(*(void **)((int)this + 0x28),this_00);
        uVar5 = 0;
        if (uVar4 != 0) {
          uVar5 = FUN_0053a1e0(this_00,piVar1);
        }
      }
      local_18 = local_18 + 1;
      uVar4 = uVar5 & local_18 < (int)param_1;
    }
  }
  ExceptionList = local_c;
  return uVar5;
}

