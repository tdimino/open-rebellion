
void __thiscall FUN_005ba670(void *this,undefined4 *param_1)

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
  undefined4 local_4;
  
  local_2c[0] = *(undefined4 *)((int)this + 0x18);
  local_2c[2] = *(undefined4 *)((int)this + 0x5c);
  local_2c[3] = *(undefined4 *)((int)this + 0x60);
  local_1c = *(undefined4 *)((int)this + 0x68);
  local_18 = 0;
  local_14 = 0;
  local_c = 0;
  local_10 = 0;
  local_8 = 0;
  local_4 = 2;
  if (*(int *)((int)this + 0x78) != 0) {
    local_4 = 9;
  }
  puVar2 = local_2c;
  for (iVar1 = 0xb; iVar1 != 0; iVar1 = iVar1 + -1) {
    *param_1 = *puVar2;
    puVar2 = puVar2 + 1;
    param_1 = param_1 + 1;
  }
  return;
}

