
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005b0280(void *this,undefined4 *param_1)

{
  int iVar1;
  undefined4 *puVar2;
  undefined4 local_2c [4];
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  undefined4 local_c;
  undefined4 local_8;
  int local_4;
  
  local_2c[2] = *(undefined4 *)((int)this + 0x128);
  local_18 = *(undefined4 *)((int)this + 0x36c);
  local_2c[0] = *(undefined4 *)((int)this + 0xec);
  local_c = *(undefined4 *)((int)this + 0x378);
  local_2c[3] = *(undefined4 *)((int)this + 300);
  local_8 = *(undefined4 *)((int)this + 0x37c);
  local_14 = *(undefined4 *)((int)this + 0x370);
  local_10 = *(undefined4 *)((int)this + 0x374);
  if (*(int *)((int)this + 0x170) == 0xb) {
    local_1c = 0;
  }
  else if (_DAT_0066c488 <= *(float *)((int)this + 0x3cc)) {
    local_1c = *(undefined4 *)((int)this + 0x3cc);
  }
  else {
    local_1c = 0x3f800000;
  }
  local_4 = *(int *)((int)this + 0x170);
  puVar2 = local_2c;
  for (iVar1 = 0xb; iVar1 != 0; iVar1 = iVar1 + -1) {
    *param_1 = *puVar2;
    puVar2 = puVar2 + 1;
    param_1 = param_1 + 1;
  }
  return;
}

