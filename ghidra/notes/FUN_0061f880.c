
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

bool __cdecl FUN_0061f880(uint param_1,double *param_2,uint param_3)

{
  ulonglong uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  uint uVar4;
  bool bVar5;
  float10 fVar6;
  int local_14;
  undefined8 local_10;
  
  uVar4 = param_1 & 0x1f;
  if (((param_1 & 8) == 0) || ((param_3 & 1) == 0)) {
    if (((param_1 & 4) == 0) || ((param_3 & 4) == 0)) {
      if (((param_1 & 1) == 0) || ((param_3 & 8) == 0)) {
        if (((param_1 & 2) != 0) && ((param_3 & 0x10) != 0)) {
          bVar5 = (param_1 & 0x10) != 0;
          local_10 = *param_2;
          if (local_10 == _DAT_0066e868) {
            bVar5 = true;
          }
          else {
            fVar6 = FUN_0061fda0(*(uint *)param_2,*(uint *)((int)param_2 + 4),&local_14);
            local_14 = local_14 + -0x600;
            if (local_14 < -0x432) {
              bVar5 = true;
              *(undefined4 *)param_2 = 0;
              local_10 = 0.0;
              *(undefined4 *)((int)param_2 + 4) = 0;
            }
            else {
              local_10 = (double)(ulonglong)
                                 (SUB87((double)fVar6,0) & 0xfffffffffffff | 0x10000000000000);
              if (local_14 < -0x3fd) {
                local_14 = -0x3fd - local_14;
                do {
                  if ((((ulonglong)local_10 & 1) != 0) && (!bVar5)) {
                    bVar5 = true;
                  }
                  uVar4 = (uint)local_10 >> 1;
                  uVar1 = (ulonglong)local_10 & 0x100000000;
                  local_10._0_4_ = uVar4;
                  if (uVar1 != 0) {
                    local_10._0_4_ = uVar4 | 0x80000000;
                  }
                  local_14 = local_14 + -1;
                  local_10 = (double)CONCAT44(local_10._4_4_ >> 1,(uint)local_10);
                } while (local_14 != 0);
              }
              if ((double)fVar6 < _DAT_0066e868) {
                local_10 = -local_10;
              }
              *(uint *)param_2 = (uint)local_10;
              *(uint *)((int)param_2 + 4) = local_10._4_4_;
            }
          }
          if (bVar5) {
            FUN_0061ff10();
          }
          uVar4 = param_1 & 0x1d;
        }
      }
      else {
        FUN_0061ff10();
        uVar3 = DAT_006af4ac;
        uVar2 = DAT_006af49c;
        uVar4 = param_3 & 0xc00;
        if (uVar4 < 0x401) {
          if (uVar4 == 0x400) {
            if (*param_2 <= _DAT_0066e868) {
              local_10 = -(double)CONCAT44(DAT_006af49c,DAT_006af498);
              *param_2 = local_10;
              uVar4 = param_1 & 0x1e;
            }
            else {
              *(undefined4 *)param_2 = DAT_006af4a8;
              *(undefined4 *)((int)param_2 + 4) = uVar3;
              uVar4 = param_1 & 0x1e;
            }
            goto LAB_0061fb8b;
          }
          if (uVar4 == 0) {
            if (*param_2 <= _DAT_0066e868) {
              local_10 = -(double)CONCAT44(DAT_006af49c,DAT_006af498);
              *param_2 = local_10;
              uVar4 = param_1 & 0x1e;
            }
            else {
              *(undefined4 *)param_2 = DAT_006af498;
              *(undefined4 *)((int)param_2 + 4) = uVar2;
              uVar4 = param_1 & 0x1e;
            }
            goto LAB_0061fb8b;
          }
        }
        else if (uVar4 == 0x800) {
          if (_DAT_0066e868 < *param_2) {
            *(undefined4 *)param_2 = DAT_006af498;
            *(undefined4 *)((int)param_2 + 4) = uVar2;
            uVar4 = param_1 & 0x1e;
            goto LAB_0061fb8b;
          }
          local_10 = -(double)CONCAT44(DAT_006af4ac,DAT_006af4a8);
          *param_2 = local_10;
        }
        else if (uVar4 == 0xc00) {
          if (*param_2 <= _DAT_0066e868) {
            local_10 = -(double)CONCAT44(DAT_006af4ac,DAT_006af4a8);
            *param_2 = local_10;
            uVar4 = param_1 & 0x1e;
          }
          else {
            *(undefined4 *)param_2 = DAT_006af4a8;
            *(undefined4 *)((int)param_2 + 4) = uVar3;
            uVar4 = param_1 & 0x1e;
          }
          goto LAB_0061fb8b;
        }
        uVar4 = param_1 & 0x1e;
      }
    }
    else {
      FUN_0061ff10();
      uVar4 = param_1 & 0x1b;
    }
  }
  else {
    FUN_0061ff10();
    uVar4 = param_1 & 0x17;
  }
LAB_0061fb8b:
  if (((param_1 & 0x10) != 0) && ((param_3 & 0x20) != 0)) {
    FUN_0061ff10();
    uVar4 = uVar4 & 0xffffffef;
  }
  return uVar4 == 0;
}

