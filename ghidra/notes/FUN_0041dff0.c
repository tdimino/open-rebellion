
int __thiscall FUN_0041dff0(void *this,uint *param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  uint uVar2;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  int iVar3;
  int local_4;
  
  local_4 = 0;
  switch(*(undefined4 *)((int)this + 8)) {
  case 0:
    iVar3 = FUN_0041e390((int)this);
    if (iVar3 != 0) {
      *(int *)((int)this + 8) = *(int *)((int)this + 8) + 1;
    }
    break;
  case 1:
    iVar3 = FUN_0041e410();
    if (iVar3 != 0) {
      *(int *)((int)this + 8) = *(int *)((int)this + 8) + 1;
    }
    break;
  case 2:
    bVar1 = FUN_0041e300((int)this);
    if (CONCAT31(extraout_var,bVar1) == 0) {
      *(int *)((int)this + 8) = *(int *)((int)this + 8) + 1;
    }
    break;
  case 3:
    uVar2 = thunk_FUN_0041cf40();
    if (uVar2 != 0) {
      *(int *)((int)this + 8) = *(int *)((int)this + 8) + 1;
    }
    break;
  case 4:
    iVar3 = FUN_0041e430();
    if (iVar3 != 0) {
      *(int *)((int)this + 8) = *(int *)((int)this + 8) + 1;
    }
    break;
  case 5:
    iVar3 = FUN_0041e460();
    if (iVar3 != 0) {
      *(int *)((int)this + 8) = *(int *)((int)this + 8) + 1;
    }
    break;
  case 6:
    iVar3 = FUN_0041e490();
    if (iVar3 != 0) {
      *(int *)((int)this + 8) = *(int *)((int)this + 8) + 1;
    }
    break;
  case 7:
    uVar2 = thunk_FUN_0041d0a0();
    if (uVar2 != 0) {
      *(int *)((int)this + 8) = *(int *)((int)this + 8) + 1;
    }
    break;
  case 8:
    iVar3 = FUN_0041cf20();
    if (iVar3 != 0) {
      *(int *)((int)this + 8) = *(int *)((int)this + 8) + 1;
    }
    break;
  case 9:
    bVar1 = FUN_0041d0e0();
    if (CONCAT31(extraout_var_00,bVar1) == 0) {
      *(int *)((int)this + 8) = *(int *)((int)this + 8) + 1;
    }
    break;
  case 10:
    iVar3 = FUN_0041d370();
    if (iVar3 != 0) {
      *(int *)((int)this + 8) = *(int *)((int)this + 8) + 1;
    }
    break;
  case 0xb:
    bVar1 = FUN_0041d390();
    if (CONCAT31(extraout_var_01,bVar1) == 0) {
      iVar3 = FUN_00401080();
      if (iVar3 != 1) {
        FUN_00401940();
      }
      break;
    }
    iVar3 = FUN_00401080();
    if (iVar3 != 1) {
      FUN_00401950();
      *(int *)((int)this + 8) = *(int *)((int)this + 8) + 1;
      break;
    }
    goto LAB_0041e127;
  case 0xc:
    iVar3 = FUN_0041e4d0();
    if (iVar3 == 0) break;
LAB_0041e127:
    *(int *)((int)this + 8) = *(int *)((int)this + 8) + 1;
    break;
  case 0xd:
    iVar3 = FUN_0041e500(&local_4);
    if (iVar3 != 0) {
      *(undefined4 *)((int)this + 8) = 0;
    }
  }
  FUN_0041cf60();
  *param_1 = (uint)(*(int *)((int)this + 8) == 2);
  return local_4;
}

