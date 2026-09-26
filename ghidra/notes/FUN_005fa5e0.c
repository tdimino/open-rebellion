
/* WARNING: Restarted to delay deadcode elimination for space: stack */

undefined4 FUN_005fa5e0(void)

{
  int iVar1;
  int *piVar2;
  undefined4 uVar3;
  undefined4 uStack_58;
  int *piStack_54;
  int *piStack_50;
  undefined4 uStack_4c;
  int *piStack_48;
  uint *puVar4;
  uint uStack_30;
  undefined4 *puStack_2c;
  undefined4 uStack_28;
  int *piStack_24;
  int iStack_20;
  undefined4 local_10;
  undefined4 local_c;
  undefined4 local_4;
  
  iStack_20 = 0;
  piStack_24 = (int *)0x0;
  puStack_2c = &local_10;
  uStack_28 = 0;
  uStack_30 = 0;
  uVar3 = 0;
  local_4 = 0;
  local_10 = 0;
  local_c = 0;
  DAT_006be354 = 0;
  iVar1 = Ordinal_4();
  if (iVar1 == 0) {
    iVar1 = (**(code **)(*piStack_24 + 0x20))();
    if ((iVar1 == -0x7788ffe2) && (uStack_30 != 0)) {
      piStack_48 = (int *)0x5fa640;
      piVar2 = (int *)FUN_00618b70(uStack_30);
      if (piVar2 != (int *)0x0) {
        puVar4 = &uStack_30;
        uStack_4c = 0;
        piStack_54 = (int *)0x5fa65e;
        piStack_50 = &iStack_20;
        piStack_48 = piVar2;
        iVar1 = (**(code **)(iStack_20 + 0x20))();
        if (iVar1 == 0) {
          uStack_58 = 0;
          *(undefined4 *)(piVar2[2] + 4) = DAT_006be3b4;
          *(undefined4 *)(piVar2[2] + 0x28) = DAT_006ac294;
          piStack_54 = piVar2;
          iVar1 = (**(code **)(*puVar4 + 0x30))(puVar4);
          if (iVar1 == 0) {
            uStack_4c = 0;
            iVar1 = (**(code **)(*piStack_54 + 0xc))(piStack_54,0,&uStack_4c);
            if ((iVar1 == 0) &&
               (iVar1 = (*(code *)*puRam00000000)(0,&DAT_0066e488,&uStack_58), iVar1 == 0)) {
              uVar3 = 1;
              DAT_006be354 = 1;
              DAT_006be35c = 0;
              iVar1 = piVar2[2];
              DAT_006be360 = *(undefined4 *)(iVar1 + 8);
              DAT_006be364 = *(undefined4 *)(iVar1 + 0xc);
              DAT_006be368 = *(undefined4 *)(iVar1 + 0x10);
              DAT_006be36c = *(undefined4 *)(iVar1 + 0x14);
              DAT_006be358 = (uint)((*(byte *)(piVar2 + 1) & 2) != 0);
            }
          }
        }
      }
    }
  }
  return uVar3;
}

