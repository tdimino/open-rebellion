
void __thiscall FUN_004865b0(int param_1,undefined4 param_2,uint *param_3)

{
  int iVar1;
  int iVar2;
  uint uStack_10;
  int iStack_c;
  int aiStack_8 [2];
  
  iVar2 = 0;
  uStack_10 = 0;
  iStack_c = 0;
  aiStack_8[0] = 0;
  aiStack_8[1] = 0;
  FUN_00401950();
  FUN_004fcdd0((undefined4 *)(param_1 + 0x3c));
  *(undefined4 *)(param_1 + 0x38) = 0;
  FUN_004f26d0((void *)(param_1 + 0x40),param_3);
  *(undefined4 *)(param_1 + 0x44) = param_2;
  iVar1 = FUN_004fcf50(*(int *)(param_1 + 0x24),&uStack_10,&iStack_c,aiStack_8,aiStack_8 + 1);
  if ((((iVar1 != 0) && (uStack_10 == 0)) && (iStack_c != 0)) && (aiStack_8[0] != 0)) {
    iVar2 = 1;
  }
  FUN_00488d20((void *)(param_1 + 0x6c),param_3,iVar2);
  *(undefined4 *)(param_1 + 0xac) = 1;
  return;
}


