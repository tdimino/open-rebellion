
void __thiscall FUN_005b8e00(void *this,undefined4 *param_1)

{
  void *this_00;
  int *piVar1;
  int iVar2;
  undefined4 *puVar3;
  longlong lVar4;
  uint uVar5;
  undefined4 local_20 [4];
  uint local_10;
  undefined4 local_c;
  int local_8;
  undefined4 local_4;
  
  local_20[0] = *(undefined4 *)((int)this + 0xec);
  local_20[2] = *(undefined4 *)((int)this + 0x128);
  local_20[3] = *(undefined4 *)((int)this + 300);
  if (*(int *)((int)this + 0x170) == 0xb) {
    local_8 = 0;
  }
  else {
    lVar4 = __ftol();
    local_8 = 1;
    if ((int)lVar4 != 0) {
      local_8 = (int)lVar4;
    }
  }
  uVar5 = *(uint *)((int)this + 0x164);
  local_c = 0;
  local_10 = uVar5;
  if (uVar5 != 0) {
    this_00 = (void *)FUN_00596bd0();
    piVar1 = FUN_005a6360(this_00,uVar5);
    if ((piVar1 != (int *)0x0) &&
       ((*(int *)((int)this + 0x170) == 1 || (local_c = 0, *(int *)((int)this + 0x170) == 5)))) {
      local_c = 1;
    }
  }
  local_4 = *(undefined4 *)((int)this + 0x170);
  puVar3 = local_20;
  for (iVar2 = 8; iVar2 != 0; iVar2 = iVar2 + -1) {
    *param_1 = *puVar3;
    puVar3 = puVar3 + 1;
    param_1 = param_1 + 1;
  }
  return;
}

