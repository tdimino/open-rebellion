
void __fastcall FUN_0041c930(int param_1)

{
  byte bVar1;
  short sVar2;
  bool bVar3;
  short sVar4;
  byte *pbVar5;
  ushort uVar6;
  short sVar7;
  short sVar8;
  short sVar9;
  undefined4 *local_c;
  int local_8;
  
  *(undefined2 *)(param_1 + 0x24) = 0;
  *(undefined2 *)(param_1 + 0x26) = 0;
  sVar2 = *(short *)(param_1 + 0x1c);
  sVar9 = *(short *)(param_1 + 0x1e);
  *(short *)(param_1 + 0x20) = sVar2;
  *(short *)(param_1 + 0x22) = sVar9;
  if (0 < sVar9) {
    local_c = *(undefined4 **)(param_1 + 0x30);
    local_8 = (int)sVar9;
    do {
      sVar8 = 0;
      sVar7 = 0;
      pbVar5 = (byte *)*local_c;
      bVar1 = *pbVar5;
      bVar3 = true;
      sVar4 = 0;
      if (0 < sVar2) {
        do {
          sVar7 = sVar4;
          uVar6 = (ushort)*pbVar5;
          if (bVar3) {
            pbVar5 = pbVar5 + 1;
            bVar3 = false;
          }
          else {
            if (uVar6 != 0) {
              sVar7 = sVar8 + uVar6;
            }
            pbVar5 = pbVar5 + (short)uVar6 + 1;
            bVar3 = true;
            if (sVar9 < *(short *)(param_1 + 0x22)) {
              *(short *)(param_1 + 0x22) = sVar9;
            }
            if (*(short *)(param_1 + 0x26) < sVar9) {
              *(short *)(param_1 + 0x26) = sVar9;
            }
          }
          sVar8 = sVar8 + uVar6;
          sVar4 = sVar7;
        } while (sVar8 < sVar2);
      }
      if ((short)(ushort)bVar1 < *(short *)(param_1 + 0x20)) {
        *(ushort *)(param_1 + 0x20) = (ushort)bVar1;
      }
      if (*(short *)(param_1 + 0x24) < sVar7) {
        *(short *)(param_1 + 0x24) = sVar7;
      }
      local_c = local_c + 1;
      sVar9 = sVar9 + -1;
      local_8 = local_8 + -1;
    } while (local_8 != 0);
  }
  *(short *)(param_1 + 0x24) = *(short *)(param_1 + 0x24) + 1;
  *(short *)(param_1 + 0x26) = *(short *)(param_1 + 0x26) + 1;
  return;
}

