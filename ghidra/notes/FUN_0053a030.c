
uint __thiscall FUN_0053a030(void *this,int *param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  uint uVar2;
  
  uVar2 = 0;
  if (param_1 != (int *)0x0) {
    bVar1 = FUN_005406d0(param_1);
    uVar2 = 0;
    if (((CONCAT31(extraout_var,bVar1) != 0) &&
        (uVar2 = (uint)((((byte)param_1[9] ^ (byte)*(undefined4 *)((int)this + 0x24)) & 0x30) == 0),
        uVar2 != 0)) && (param_1[7] != 0)) {
      uVar2 = (uint)(*(int *)(param_1[7] + 0x28) != 0);
    }
  }
  return uVar2;
}

