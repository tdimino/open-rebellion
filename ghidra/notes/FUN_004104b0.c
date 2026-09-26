
void __fastcall FUN_004104b0(void *param_1)

{
  bool bVar1;
  undefined4 uVar2;
  int *piVar3;
  DWORD DVar4;
  uint uVar5;
  undefined3 extraout_var;
  undefined4 extraout_ECX;
  
  uVar2 = FUN_00597090();
  if ((*(int *)((int)param_1 + 0x118) == 0) && (*(int *)((int)param_1 + 0x114) == 1)) {
    *(undefined4 *)((int)param_1 + 0x138) = uVar2;
    piVar3 = (int *)FUN_00410790();
    if (piVar3 != (int *)0x0) {
      uVar2 = (**(code **)(*piVar3 + 0xc))();
      switch(uVar2) {
      case 0x10:
        DVar4 = timeGetTime();
        *(DWORD *)((int)param_1 + 0x134) = DVar4;
        if (*(int *)((int)param_1 + 0x11c) == 0) {
          FUN_0040f9f0(param_1,piVar3[0xb],0,0);
          FUN_0040fa70(param_1,piVar3[9],0,0);
          FUN_0040faf0(param_1,piVar3[10],0,0);
          *(int *)((int)param_1 + 0x130) = piVar3[8];
          if (piVar3[0xc] != 0) {
            *(undefined4 *)((int)param_1 + 0x118) = 1;
            KillTimer(*(HWND *)((int)param_1 + 0x18),0x191);
            FUN_004010a0(*(undefined4 *)((int)param_1 + 0x120),2,
                         *(undefined4 *)((int)param_1 + 0x124),*(undefined4 *)((int)param_1 + 0x130)
                         ,*(int *)((int)param_1 + 0x128),7);
            FUN_00401980(*(undefined4 *)((int)param_1 + 0x11c));
            FUN_00597440(extraout_ECX);
            if (*(int *)((int)param_1 + 0x11c) != 0) {
              FUN_0040fd20(param_1,1);
              DVar4 = timeGetTime();
              uVar5 = DVar4 - *(int *)((int)param_1 + 0x134);
              while ((uVar5 < 7000 && (bVar1 = FUN_00597140(), CONCAT31(extraout_var,bVar1) == 0)))
              {
                DVar4 = timeGetTime();
                uVar5 = DVar4 - *(int *)((int)param_1 + 0x134);
              }
            }
            KillTimer(*(HWND *)((int)param_1 + 0x18),400);
            *(undefined4 *)((int)param_1 + 0x118) = 1;
            FUN_00401040(7);
            (**(code **)*piVar3)(1);
            return;
          }
        }
        break;
      case 0x11:
        FUN_00410850(param_1,piVar3);
        (**(code **)*piVar3)(1);
        return;
      case 0x12:
        *(undefined4 *)((int)param_1 + 0x118) = 1;
        FUN_004212a0((int)piVar3);
        (**(code **)*piVar3)(1);
        return;
      case 0x14:
        if (*(int *)((int)param_1 + 0x11c) == 0) {
          FUN_004101e0(param_1);
          FUN_004114c0();
          (**(code **)*piVar3)(1);
          return;
        }
        FUN_00420d10((int)piVar3);
        FUN_004109e0((int)param_1);
        if ((DAT_006b123c & 2) == 0) {
          FUN_0040fe10(param_1);
        }
        DAT_006b1248 = 0xffffffff;
        DAT_006b123c = DAT_006b123c | 2;
        (**(code **)*piVar3)(1);
        return;
      case 0x16:
        FUN_00421080((int)piVar3);
      }
      (**(code **)*piVar3)(1);
    }
  }
  return;
}

