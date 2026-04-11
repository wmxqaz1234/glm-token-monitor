import { calculateWaterY, type WaterPathResult } from './waterLevel'

/**
 * 生成 PixelGhost 的水位路径
 * @param usagePercent 用量百分比 (0-100)
 */
export function generatePixelGhostWaterPath(usagePercent: number): WaterPathResult {
  // PixelGhost viewBox: 0 0 64 64
  const VIEW_BOX = { width: 64, height: 64, bottom: 64 }

  // 计算水位线 Y 坐标
  const waterY = calculateWaterY(usagePercent, VIEW_BOX.height, VIEW_BOX.bottom)

  // PixelGhost 身体路径的关键点
  const BODY = {
    topY: 14,           // 顶部 Y
    bottomY: 46,        // 底部波浪的最高点
    leftX: 18,          // 左侧 X
    rightX: 50,         // 右侧 X
    centerX: 32         // 中心 X
  }

  // 边界情况
  if (waterY >= BODY.bottomY) {
    // 完全空
    return {
      waterPath: '',
      emptyPath: getOriginalGhostPath(),
      isEmpty: true,
      isFull: false
    }
  }

  if (waterY <= BODY.topY) {
    // 完全满
    return {
      waterPath: getOriginalGhostPath(),
      emptyPath: '',
      isEmpty: false,
      isFull: true
    }
  }

  // 正常情况：生成水位切割路径
  const { waterPath, emptyPath } = generateSplitGhostPaths(waterY)

  return {
    waterPath,
    emptyPath,
    isEmpty: false,
    isFull: false
  }
}

/**
 * 获取原始 PixelGhost 完整路径
 */
function getOriginalGhostPath(): string {
  return `M 18 14
          Q 32 12 46 14
          Q 50 14 50 18
          L 50 42
          Q 50 46 46 46
          Q 44 48 42 46
          Q 40 44 38 46
          Q 36 48 34 46
          Q 32 44 30 46
          Q 28 48 26 46
          Q 24 44 22 46
          Q 20 48 18 46
          Q 14 46 14 42
          L 14 18
          Q 14 14 18 14
          Z`
}

/**
 * 根据水位线生成分割路径
 */
function generateSplitGhostPaths(waterY: number): { waterPath: string; emptyPath: string } {
  const leftX = 18
  const rightX = 50

  // 有水部分：从底部向上到水位线
  const waterPath = `M ${leftX} ${waterY}
                     L ${leftX} 42
                     Q 14 42 14 42
                     L 14 18
                     Q 14 14 18 14
                     Q 32 12 46 14
                     Q 50 14 50 18
                     L 50 42
                     Q 50 46 46 46
                     Q 44 48 42 46
                     Q 40 44 38 46
                     Q 36 48 34 46
                     Q 32 44 30 46
                     Q 28 48 26 46
                     Q 24 44 22 46
                     Q 20 48 18 46
                     Q 14 46 14 42
                     L ${leftX} 42
                     L ${leftX} ${waterY}
                     L ${rightX} ${waterY}
                     Z`

  // 无水部分：从水位线向上到顶部
  const emptyPath = `M 18 14
                     Q 32 12 46 14
                     Q 50 14 50 18
                     L 50 ${waterY}
                     L 18 ${waterY}
                     Q 14 14 18 14
                     Z`

  return { waterPath, emptyPath }
}
