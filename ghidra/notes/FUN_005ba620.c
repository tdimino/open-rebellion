
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005ba620(void *this,undefined4 *param_1)

{
  int iVar1;
  undefined4 *puVar2;
  undefined4 local_14 [4];
  undefined4 local_4;
  
  local_14[2] = 1;
  if (*(float *)((int)this + 0x68) <= _DAT_0066c710) {
    local_14[2] = 0;
  }
  local_14[3] = *(undefined4 *)((int)this + 0x68);
  local_4 = *(undefined4 *)((int)this + 0x78);
  puVar2 = local_14;
  for (iVar1 = 5; iVar1 != 0; iVar1 = iVar1 + -1) {
    *param_1 = *puVar2;
    puVar2 = puVar2 + 1;
    param_1 = param_1 + 1;
  }
  return;
}

