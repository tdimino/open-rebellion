
uint __thiscall FUN_004ef260(int *param_1,void *param_2)

{
  bool bVar1;
  uint uVar2;
  undefined3 extraout_var;
  int iStack_4;
  
  uVar2 = 1;
  if (((*(byte *)(param_1 + 0x1e) & 0x80) == 0) && ((*(byte *)(param_1 + 0x14) & 8) == 0)) {
    iStack_4 = 0;
    bVar1 = FUN_004f6b50(param_1,&iStack_4);
    uVar2 = (uint)(CONCAT31(extraout_var,bVar1) != 0);
    if ((iStack_4 != 0) && (((*(uint *)(iStack_4 + 0x24) ^ param_1[9]) & 0xc0) != 0)) {
      if ((*(byte *)(param_1 + 0x2b) & 1) != 0) {
        uVar2 = (**(code **)(*param_1 + 0x218))(param_2);
      }
      if (((*(uint *)(iStack_4 + 0x24) ^ param_1[9]) & 0xc0) != 0) {
        uVar2 = FUN_004f78e0(param_1,1,param_2);
      }
    }
  }
  return uVar2;
}

