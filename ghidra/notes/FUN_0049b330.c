
void __fastcall FUN_0049b330(int param_1)

{
  int *piVar1;
  int iVar2;
  int iVar3;
  int *piVar4;
  uint uVar5;
  undefined4 *puVar6;
  void *pvVar7;
  
  iVar2 = FUN_004f3dd0(*(int *)(param_1 + 0x20),*(int *)(param_1 + 0x20));
  iVar3 = FUN_004f3dd0(*(int *)(param_1 + 0x20),2 - (*(int *)(param_1 + 0x20) != 1));
  if (iVar2 != 0) {
    FUN_0060b9d0((void *)(param_1 + 0x94),0x7002,(undefined4 *)(iVar2 + 0x30),(undefined4 *)0x0,
                 (undefined4 *)0x0,(undefined4 *)0x0);
  }
  piVar4 = FUN_004f3220(*(int *)(param_1 + 0x20),(uint *)(param_1 + 0x44));
  if (((iVar2 != 0) && (piVar4 != (int *)0x0)) && (iVar3 != 0)) {
    piVar1 = piVar4 + 0xc;
    FUN_0060b9d0((void *)(param_1 + 0x88),0x7001,piVar1,(undefined4 *)0x0,(undefined4 *)0x0,
                 (undefined4 *)0x0);
    puVar6 = (undefined4 *)(iVar2 + 0x30);
    FUN_0060b9d0((void *)(param_1 + 0xac),0x7012,puVar6,piVar1,(undefined4 *)0x0,(undefined4 *)0x0);
    FUN_0060b9d0((void *)(param_1 + 0xb8),0x7013,(undefined4 *)0x0,(undefined4 *)0x0,
                 (undefined4 *)0x0,(undefined4 *)0x0);
    uVar5 = (uint)piVar4[9] >> 6 & 3;
    if ((uVar5 == 0) || (uVar5 == 3)) {
      puVar6 = (undefined4 *)(iVar3 + 0x30);
      uVar5 = 0x7007;
    }
    else if (*(uint *)(param_1 + 0x5c) == uVar5) {
      if (uVar5 == *(uint *)(param_1 + 0x58)) {
        uVar5 = 0x7016;
      }
      else {
        uVar5 = 0x7015;
      }
    }
    else if (uVar5 == *(uint *)(param_1 + 0x58)) {
      puVar6 = (undefined4 *)(iVar3 + 0x30);
      uVar5 = 0x7005;
    }
    else {
      uVar5 = 0x7006;
    }
    pvVar7 = (void *)(param_1 + 0xa0);
    FUN_0060b9d0(pvVar7,uVar5,piVar1,puVar6,(undefined4 *)0x0,(undefined4 *)0x0);
    if (*(int *)(param_1 + 0x54) == 0) {
      FUN_0060b9d0(pvVar7,0x700b,(undefined4 *)(iVar3 + 0x30),(undefined4 *)0x0,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      return;
    }
    FUN_0060b9d0(pvVar7,0x700c,(undefined4 *)(iVar3 + 0x30),(undefined4 *)0x0,(undefined4 *)0x0,
                 (undefined4 *)0x0);
  }
  return;
}

