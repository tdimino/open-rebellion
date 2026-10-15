
void __thiscall FUN_00402e40(int param_1,uint *param_2)

{
  uint uVar1;
  
  uVar1 = *param_2 & 0xffffff | *(int *)(param_1 + 0x2c) << 0x18;
  *param_2 = (*(uint *)(param_1 + 0x18) ^ uVar1) & 0xffffff ^ uVar1;
  return;
}

