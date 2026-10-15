
void __fastcall FUN_004e5530(int *param_1)

{
  uint uVar1;
  
  uVar1 = FUN_005f4960((undefined4 *)(param_1[3] + 4));
  if ((uVar1 & 2) != 0) {
    param_1[4] = param_1[4] & 0xfffffffe;
    (**(code **)(*param_1 + 0x14))();
    *(uint *)(param_1[3] + 4) = *(uint *)(param_1[3] + 4) & 0xfffffffd;
  }
  return;
}

