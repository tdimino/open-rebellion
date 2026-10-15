
void __thiscall FUN_004f7ed0(int *param_1,undefined4 param_2)

{
  int iVar1;
  uint uVar2;
  int *local_4;
  
  local_4 = param_1;
  iVar1 = FUN_004f6b50(&local_4);
  uVar2 = 0;
  if (iVar1 != 0) {
    uVar2 = (uint)local_4[0x14] >> 3 & 1;
  }
  if (uVar2 != 0) {
    (**(code **)(*param_1 + 0xac))(local_4[0x10] & 0xff,param_2);
  }
  return;
}

