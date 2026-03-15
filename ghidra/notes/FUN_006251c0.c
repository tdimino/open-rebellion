
void __cdecl FUN_006251c0(int param_1,int param_2)

{
  byte bVar1;
  int iVar2;
  int iVar3;
  uint *puVar4;
  
  bVar1 = (byte)(param_2 >> 0x1f);
  iVar3 = (int)(param_2 + (param_2 >> 0x1f & 0x1fU)) >> 5;
  iVar2 = FUN_00627580(*(uint *)(param_1 + iVar3 * 4),
                       1 << (0x1f - ((((byte)param_2 ^ bVar1) - bVar1 & 0x1f ^ bVar1) - bVar1) &
                            0x1f),(uint *)(param_1 + iVar3 * 4));
  iVar3 = iVar3 + -1;
  if (-1 < iVar3) {
    puVar4 = (uint *)(param_1 + iVar3 * 4);
    do {
      if (iVar2 == 0) {
        return;
      }
      iVar2 = FUN_00627580(*puVar4,1,puVar4);
      iVar3 = iVar3 + -1;
      puVar4 = puVar4 + -1;
    } while (-1 < iVar3);
  }
  return;
}

