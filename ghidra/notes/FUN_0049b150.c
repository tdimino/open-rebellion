
void __fastcall FUN_0049b150(int param_1)

{
  undefined4 *puVar1;
  void *pvVar2;
  int iVar3;
  int iVar4;
  int *piVar5;
  int *piVar6;
  undefined4 *puVar7;
  uint uVar8;
  
  iVar3 = FUN_004f3dd0(*(int *)(param_1 + 0x20),*(int *)(param_1 + 0x20));
  iVar4 = FUN_004f3dd0(*(int *)(param_1 + 0x20),2 - (*(int *)(param_1 + 0x20) != 1));
  piVar5 = FUN_004f3220(*(int *)(param_1 + 0x20),(uint *)(param_1 + 0x44));
  if (iVar3 == 0) {
    return;
  }
  if (piVar5 == (int *)0x0) {
    return;
  }
  if (iVar4 == 0) {
    return;
  }
  piVar5 = piVar5 + 0xc;
  FUN_0060b9d0((void *)(param_1 + 0x88),0x7001,piVar5,(undefined4 *)0x0,(undefined4 *)0x0,
               (undefined4 *)0x0);
  FUN_0060b9d0((void *)(param_1 + 0x94),0x7004,piVar5,(undefined4 *)0x0,(undefined4 *)0x0,
               (undefined4 *)0x0);
  puVar1 = (undefined4 *)(iVar4 + 0x30);
  puVar7 = (undefined4 *)(iVar3 + 0x30);
  pvVar2 = (void *)(param_1 + 0xa0);
  FUN_0060b9d0(pvVar2,0x700e,puVar7,puVar1,(undefined4 *)0x0,(undefined4 *)0x0);
  if (*(int *)(param_1 + 0x54) == 0) {
    FUN_0060b9d0(pvVar2,0x700f,puVar7,puVar1,(undefined4 *)0x0,(undefined4 *)0x0);
  }
  else {
    if (*(int *)(param_1 + 0x54) == *(int *)(param_1 + 0x58)) {
      piVar6 = FUN_004f3220(*(int *)(param_1 + 0x20),(uint *)(param_1 + 0x48));
      if (piVar6 == (int *)0x0) {
        return;
      }
      FUN_0060b9d0(pvVar2,0x700d,puVar7,piVar6 + 0xc,(undefined4 *)0x0,(undefined4 *)0x0);
      FUN_0060b9d0(pvVar2,0x700b,puVar1,(undefined4 *)0x0,(undefined4 *)0x0,(undefined4 *)0x0);
      FUN_0060b9d0((void *)(param_1 + 0xac),0x7011,puVar7,piVar5,piVar6 + 0xc,(undefined4 *)0x0);
      uVar8 = 0x7013;
      goto LAB_0049b30c;
    }
    FUN_0060b9d0(pvVar2,0x700b,puVar7,(undefined4 *)0x0,(undefined4 *)0x0,(undefined4 *)0x0);
    FUN_0060b9d0(pvVar2,0x700c,puVar1,(undefined4 *)0x0,(undefined4 *)0x0,(undefined4 *)0x0);
  }
  FUN_0060b9d0((void *)(param_1 + 0xac),0x7010,puVar7,piVar5,(undefined4 *)0x0,(undefined4 *)0x0);
  uVar8 = 0x7014;
LAB_0049b30c:
  FUN_0060b9d0((void *)(param_1 + 0xb8),uVar8,(undefined4 *)0x0,(undefined4 *)0x0,(undefined4 *)0x0,
               (undefined4 *)0x0);
  return;
}

