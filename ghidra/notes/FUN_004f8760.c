
void __thiscall FUN_004f8760(int *param_1,undefined4 param_2)

{
  uint uVar1;
  int *piStack_4;
  
  piStack_4 = param_1;
  FUN_004f6b50(&piStack_4);
  uVar1 = param_1[0x14];
  if (((uVar1 & 4) != 0) && ((uVar1 & 0x20) == 0)) {
    (**(code **)(*param_1 + 0xac))(4,param_2);
    return;
  }
  if ((uVar1 & 8) == 0) {
    FUN_004f78e0(1,param_2);
  }
  return;
}

